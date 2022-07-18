#include "tcp_connection.hh"

#include "tcp_config.hh"
#include "tcp_segment.hh"

#include <cstddef>
#include <iostream>

// Dummy implementation of a TCP connection

// For Lab 4, please replace with a real implementation that passes the
// automated checks run by `make check`.

template <typename... Targs>
void DUMMY_CODE(Targs &&.../* unused */) {}

using namespace std;

void TCPConnection::push_segment_out(bool send_syn) {
    _sender.fill_window(send_syn || in_recv());
    TCPSegment seg;
    while (!_sender.segments_out().empty()) {
        seg = _sender.segments_out().front();
        _sender.segments_out().pop();

        // if _receiver has value
        if (_receiver.ackno().has_value()) {
            seg.header().ack = true;
            seg.header().ackno = _receiver.ackno().value();
            seg.header().win = _receiver.window_size();
        }

        if (_to_send_rst) {
            _to_send_rst = false;
            seg.header().rst = true;
        }
        _segments_out.push(seg);
    }
}

size_t TCPConnection::remaining_outbound_capacity() const { return _sender.stream_in().remaining_capacity(); }

size_t TCPConnection::bytes_in_flight() const { return _sender.bytes_in_flight(); }

size_t TCPConnection::unassembled_bytes() const { return _receiver.unassembled_bytes(); }

size_t TCPConnection::time_since_last_segment_received() const { return _time_since_last_segment_received; }

void TCPConnection::segment_received(const TCPSegment &seg) {
    // error happen or not an active conncetion
    if (!_active) {
        return;
    }
    _time_since_last_segment_received = 0;

    // A BIG FSM

    // fisrt we consider very special conditon
    if (in_syn_sent()) {
        if (seg.header().ack) {
        }
    }
}

bool TCPConnection::active() const { return _active; }

size_t TCPConnection::write(const string &data) {
  size_t ret = _sender.stream_in().write(data);
  push_segment_out();
  return ret;
}

//! \param[in] ms_since_last_tick number of milliseconds since the last call to this method
void TCPConnection::tick(const size_t ms_since_last_tick) {
  if(!_active)
    return;

  _time_since_last_segment_received +=ms_since_last_tick;
  _sender.tick(ms_since_last_tick);
  if (_sender.consecutive_retransmissions()>TCPConfig::MAX_RETX_ATTEMPTS) {
    unclen_shutdown(true);
  }
  push_segment_out();
}

void TCPConnection::end_input_stream() {}

void TCPConnection::connect() { push_segment_out(true); }

TCPConnection::~TCPConnection() {
    try {
        if (active()) {
            cerr << "Warning: Unclean shutdown of TCPConnection\n";

            // Your code here: need to send a RST segment to the peer
        }
    } catch (const exception &e) {
        std::cerr << "Exception destructing TCP FSM: " << e.what() << std::endl;
    }
}
