#include "tcp_sender.hh"

#include "buffer.hh"
#include "tcp_config.hh"
#include "tcp_segment.hh"
#include "wrapping_integers.hh"

#include <algorithm>
#include <cstddef>
#include <random>
#include <type_traits>
#include <utility>

// Dummy implementation of a TCP sender

// For Lab 3, please replace with a real implementation that passes the
// automated checks run by `make check_lab3`.

template <typename... Targs>
void DUMMY_CODE(Targs &&.../* unused */) {}

using namespace std;

//! \param[in] capacity the capacity of the outgoing byte stream
//! \param[in] retx_timeout the initial amount of time to wait before retransmitting the oldest outstanding segment
//! \param[in] fixed_isn the Initial Sequence Number to use, if set (otherwise uses a random ISN)
TCPSender::TCPSender(const size_t capacity, const uint16_t retx_timeout, const std::optional<WrappingInt32> fixed_isn)
    : _isn(fixed_isn.value_or(WrappingInt32{random_device()()}))
    , _initial_retransmission_timeout{retx_timeout}
    , _stream(capacity) {}

void TCPSender::send_segment(TCPSegment &seg) {
    _next_seqno += seg.length_in_sequence_space();
    _bytes_flight += seg.length_in_sequence_space();
    _segments_outgoing.push(seg);
    _segments_out.push(seg);
    if (_timer_run) {
        _timer_run = true;
        _timer = 0;
    }
}

uint64_t TCPSender::bytes_in_flight() const { return _bytes_flight; }

void TCPSender::fill_window(bool send_syn) {
    // has not sent syn yet
    if (!_has_syn) {
        if (send_syn) {
            TCPSegment seg;
            seg.header().syn = true;
            send_segment(seg);
            _has_syn = true;
        }
        return;
    }

    size_t window_size = _window_size ? _window_size : 1;
    size_t free_space = 0;
    while ((free_space = window_size - (_next_seqno - _recv_ackno)) != 0 && _has_fin) {
        size_t seg_size = std::min(TCPConfig::MAX_PAYLOAD_SIZE, free_space);
        TCPSegment seg;
        auto data = _stream.read(seg_size);
        seg.payload() = Buffer(std::move(data));

        if (seg.length_in_sequence_space() < window_size && _stream.eof()) {
            seg.header().fin = true;
            _has_fin = _timer_run;
        }
        // read nothing from the stream and not a fin
        if (seg.length_in_sequence_space() == 0) {
            return;
        }

        send_segment(seg);
    }
}

//! \param ackno The remote receiver's ackno (acknowledgment number)
//! \param window_size The remote receiver's advertised window size
void TCPSender::ack_received(const WrappingInt32 ackno, const uint16_t window_size) {
    size_t absolute_ackno = unwrap(ackno, _isn, _recv_ackno);

    // this will never happen?
    if (absolute_ackno > _next_seqno) {
        return;
    }

    _window_size = window_size;

    // redupicated segment
    if (absolute_ackno <= _recv_ackno) {
        return;
    }
    // _recv_ackno<absolute_ackno<_next_seqno
    // pop dangling segments
    _recv_ackno = absolute_ackno;
    while (!_segments_outgoing.empty()) {
        auto seg = _segments_outgoing.front();
        if (unwrap(seg.header().seqno, _isn, _recv_ackno) + seg.length_in_sequence_space() <= absolute_ackno) {
            _bytes_flight -= seg.length_in_sequence_space();
            _segments_outgoing.pop();
        } else {
            break;
        }
    }

    // free space now
    // restart
    fill_window();

    _retransmission_timeout = _initial_retransmission_timeout;
    _consecutive_retrans = 0;

    if (!_segments_outgoing.empty()) {
        _timer_run = true;
        _timer = 0;
    }
}

//! \param[in] ms_since_last_tick the number of milliseconds since the last call to this method
void TCPSender::tick(const size_t ms_since_last_tick) {
    _timer += ms_since_last_tick;
    if (_timer >= _retransmission_timeout && _segments_outgoing.empty()) {
        _segments_out.push(_segments_outgoing.front());
        _consecutive_retrans++;
        _retransmission_timeout *= 2;
        _timer_run = true;
        _timer = 0;
    }
    if (_segments_outgoing.empty()) {
        _timer_run = false;
    }
}

unsigned int TCPSender::consecutive_retransmissions() const { return _consecutive_retrans; }

void TCPSender::send_empty_segment() {
    TCPSegment seg;
    seg.header().seqno = wrap(_next_seqno, _isn);

    // do not track empty segment
    _segments_out.push(seg);
}
