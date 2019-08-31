# mIRCo

The irc client

## Usage
```bash
cargo run --bin mirco server nick name
```
## Test server
```bash
docker run --name ircd -p 6667:6667 inspircd/inspircd-docker
```
## Specs

* <https://tools.ietf.org/rfc/rfc1459.txt>
* <https://tools.ietf.org/rfc/rfc2812.txt>

## Info

* <https://www.alien.net.au/irc/irc2numerics.html>
* <http://blog.initprogram.com/2010/10/14/a-quick-basic-primer-on-the-irc-protocol/>
* <https://shiroyasha.svbtle.com/escape-sequences-a-quick-guide-1>
* <https://github.com/krpors/hx/blob/develop/util.c#L213>
* <https://mike42.me/blog/2018-06-make-better-cli-progress-bars-with-unicode-block-characters>
* <http://www.irchelp.org/networks/servers/ircnet.html>
* <https://www.csie.ntu.edu.tw/~r92094/c++/VT100.html>
