var searchIndex = {};
searchIndex["nonblock"] = {"doc":"Read available data from file descriptors without blocking","items":[[3,"NonBlockingReader","nonblock","Simple non-blocking wrapper for reader types that implement AsRawFd",null,null],[11,"from_fd","","Initialize a NonBlockingReader from the reader&#39;s file descriptor.",0,{"inputs":[{"name":"r"}],"output":{"name":"result"}}],[11,"into_blocking","","Consume this NonBlockingReader and return the blocking version\n  of the interanally managed reader.",0,{"inputs":[{"name":"nonblockingreader"}],"output":{"name":"result"}}],[11,"is_eof","","Indicates if EOF has been reached for the reader.",0,{"inputs":[{"name":"nonblockingreader"}],"output":{"name":"bool"}}],[11,"read_available","","Reads any available data from the reader without blocking, placing them into `buf`.",0,{"inputs":[{"name":"nonblockingreader"},{"name":"vec"}],"output":{"name":"result"}}],[11,"read_available_to_string","","Reads any available data from the reader without blocking, placing them into `buf`.",0,{"inputs":[{"name":"nonblockingreader"},{"name":"string"}],"output":{"name":"result"}}]],"paths":[[3,"NonBlockingReader"]]};
initSearch(searchIndex);
