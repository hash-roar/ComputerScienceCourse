#include "tcp_receiver.hh"

#include "wrapping_integers.hh"

#include <cstddef>
#include <optional>

// Dummy implementation of a TCP receiver

// For Lab 2, please replace with a real implementation that passes the
// automated checks run by `make check_lab2`.

template <typename... Targs>
void DUMMY_CODE(Targs &&.../* unused */) {}

using namespace std;

void TCPReceiver::segment_received(const TCPSegment &seg) {
    // init some variables

    size_t recv_length = 0;  // receive data length
    bool return_flag = true;

    // especilly consider SYN and Fin packet
    if (seg.header().ack) {
        // another syn
        if (_has_syn) {
            return;
        }

        // first syn packet
        _has_syn = true;
        _isn = seg.header().seqno.raw_value();
        _want_index = 1;

        recv_length = seg.length_in_sequence_space() - 1;

        if (recv_length == 0) {
            return;
        }

        // dicard al syn packet
    } else if (_has_syn == false) {
        return;
    } else {
        _absolue_index = unwrap(WrappingInt32{seg.header().seqno.raw_value()}, WrappingInt32{_isn}, _absolue_index);
        recv_length = seg.length_in_sequence_space() - 1;
    }

    if (seg.header().fin) {
        // discard duplicated FIN  packet
        if (_has_fin) {
            return;
        }

        return_flag = false;
        _has_fin = true;
        // may have data
    } else if (_absolue_index >= _want_index + _want_index || _absolue_index + recv_length <= _want_index) {
        // fin packet can not return here
        if (!return_flag) {
            return;
        }
    }

    _reassembler.push_substring(seg.payload().copy(), _absolue_index - 1, seg.header().fin);
    _want_index = _reassembler.want_index() + 1;
    if (_reassembler.input_ended()) {
        _absolue_index++;
    }
    return;

    // check windows and push data
}

optional<WrappingInt32> TCPReceiver::ackno() const {
    if (_want_index) {
        return WrappingInt32(wrap(_want_index, WrappingInt32{_isn}));
    } else {
        return std::nullopt;
    }
}

size_t TCPReceiver::window_size() const {
    return _capacity - _reassembler.stream_out().buffer_size() - _reassembler.unassembled_bytes();
}
