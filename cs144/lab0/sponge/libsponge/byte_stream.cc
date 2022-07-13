#include "byte_stream.hh"

#include <algorithm>
#include <assert.h>
#include <cstddef>
#include <string>

// Dummy implementation of a flow-controlled in-memory byte stream.

// For Lab 0, please replace with a real implementation that passes the
// automated checks run by `make check_lab0`.

// You will need to add private members to the class declaration in `byte_stream.hh`

template <typename... Targs>
void DUMMY_CODE(Targs &&.../* unused */) {}

using namespace std;

ByteStream::ByteStream(const size_t capacity) {
    _ringbuffer.resize(capacity);
    _capacity = capacity;
    _head = 0;
    _tail = 0;
    _total_written = 0;
    _flag = false;
    _eof = false;
    _input_end = false;
}

size_t ByteStream::write(const string &data) {
    size_t data_size = data.size();
    size_t write_len = 0;
    if (data_size <= remaining_capacity()) {
        write_len = data_size;
    } else {
        write_len = remaining_capacity();
    }
    while (write_len) {
        // this will throw
        // we use at instead of operator[] reasonably
        _ringbuffer[_tail] = data.at(data_size - write_len);
        _tail = (_tail + 1) % _capacity;
        write_len--;
    }
    return write_len;
}

//! \param[in] len bytes will be copied from the output side of the buffer
string ByteStream::peek_output(const size_t len) const {
    assert(len <= buffer_size());
    std::string result(len+1,'\0');
    size_t write_len =len;
    auto head_pos = _head;
    while (write_len) {
      result[len-write_len] = _ringbuffer[head_pos];
      head_pos = (head_pos+1)%_capacity;
      write_len--;
    }

    return result;
}

//! \param[in] len bytes will be removed from the output side of the buffer
void ByteStream::pop_output(const size_t len) {
  assert(len <= buffer_size());
  auto pop_len = len;
  while (pop_len) {
    _head=(_head+1)%_capacity;
    pop_len--;
  }
}

//! Read (i.e., copy and then pop) the next "len" bytes of the stream
//! \param[in] len bytes will be popped and returned
//! \returns a string
std::string ByteStream::read(const size_t len) {
  auto result = peek_output(len);

  peek_output(len);
  return result;
}

void ByteStream::end_input() { _input_end = true; }

bool ByteStream::input_ended() const { return _input_end; }

size_t ByteStream::buffer_size() const {
    if (_tail > _head) {
        return _tail - _head;
    }  // most easy case
    else if (_tail == _head) {
        return _flag ? _capacity : 0;
    } else {
        return _tail + _capacity - _head;
    }
}

bool ByteStream::buffer_empty() const { return _head == _tail && _flag == false; }

bool ByteStream::eof() const { return _eof; }

size_t ByteStream::bytes_written() const { return _total_written; }

size_t ByteStream::bytes_read() const { return _total_written - buffer_size(); }

size_t ByteStream::remaining_capacity() const { return _capacity - buffer_size(); }
