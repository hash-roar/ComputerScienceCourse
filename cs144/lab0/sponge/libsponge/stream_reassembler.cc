#include "stream_reassembler.hh"

#include <cstddef>

// Dummy implementation of a stream reassembler.

// For Lab 1, please replace with a real implementation that passes the
// automated checks run by `make check_lab1`.

// You will need to add private members to the class declaration in `stream_reassembler.hh`

template <typename... Targs>
void DUMMY_CODE(Targs &&.../* unused */) {}

using namespace std;

StreamReassembler::StreamReassembler(const size_t capacity) : _capacity(capacity), _output(capacity) {}

//! \details This function accepts a substring (aka a segment) of bytes,
//! possibly out-of-order, from the logical stream, and assembles any newly
//! contiguous substrings and writes them into the output stream in order.
void StreamReassembler::push_substring(const string &data, const size_t index, const bool eof) {
    // divide the capacity
    if (_output.buffer_size() + _unassembled_bytes + data.size() > _capacity) {
        return;
    }
    string_node dummy_node;
    // we have sufficeint space here
    if (index + data.size() <= _wanted_index) {
        return;
    } else if (index < _wanted_index) {
        dummy_node.index = _wanted_index;
        dummy_node.data.assign(data.begin() + (_wanted_index - index), data.end());
        dummy_node.len = dummy_node.data.length();
    } else {
        dummy_node.index = index;
        dummy_node.data = data;
        dummy_node.len = data.length();
    }

    // we should merge nodes untill no more
    do {
        int merge_bytes = 0;
        auto iter = _string_nodes.lower_bound(dummy_node);
        // merge with next nodes
        while (iter != _string_nodes.end() && (merge_bytes = merge_nodes(dummy_node, *iter)) >= 0) {
            _unassembled_bytes -= merge_bytes;
            _string_nodes.erase(iter);
            iter = _string_nodes.lower_bound(dummy_node);
        }

        if (iter == _string_nodes.begin()) {
            return;
        }
        // merge with pre nodes
        while ((merge_bytes = merge_nodes(dummy_node, *iter)) >= 0) {
            _unassembled_bytes -= merge_bytes;
            _string_nodes.erase(iter);
            iter = _string_nodes.lower_bound(dummy_node);
            if (iter == _string_nodes.begin()) {
                return;
            }
            iter--;
        }

    } while (false);

    _string_nodes.insert(dummy_node);

    // write to byte_stream
    if (_string_nodes.empty() && _string_nodes.begin()->index == _wanted_index) {
        size_t write_bytes = _output.write(_string_nodes.begin()->data);
        _wanted_index += write_bytes;
        _unassembled_bytes -= write_bytes;
        _string_nodes.erase(_string_nodes.begin());
    }
}

int StreamReassembler::merge_nodes(string_node &node1, const string_node &node2) {
    // order node1 and node2
    string_node temp1, temp2;
    if (node1.index > node2.index) {
        temp1 = node2;
        temp2 = node1;
    } else {
        temp1 = node1;
        temp2 = node2;
    }

    if (temp1.index+temp1.len <=temp2.index) {
      return -1;
    }else if (temp1.index+temp1.len<temp2.index+temp2.len) {
      node1.index = temp1.index;
      node1.data = temp1.data+temp2.data.substr(temp1.index+temp1.len-temp2.index);
      node1.len = node1.data.length();
      return temp1.index+temp1.len-temp2.index;
    }else {
      node1 = temp1;
      return temp2.len;
    }
}

size_t StreamReassembler::unassembled_bytes() const { return {}; }

bool StreamReassembler::empty() const { return {}; }
