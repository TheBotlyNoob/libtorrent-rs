use autocxx::prelude::*;

include_cpp! {
    #include "libtorrent/session.hpp"
    safety!(unsafe)
    generate_ns!("libtorrent")
}
