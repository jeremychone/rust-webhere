Very simple way to serve a local web server from a directory.

Used for dev. Do not use for prod. 

```sh
# Start default port at current dir: http://localhost:8080 at the current folder
webhere

# Starts with custom port: http://localhost:8888
webhere -p 8888

# Start at custom root dir
webhere -d /some/dir

# Start with live mode
#   Will add `<script src="/_webhere_live.js"></script>` at the end of all html file)
#   A web-socket server is always on at /_webhere_live_ws (which send events when root dir file changes)
webhere -l
```