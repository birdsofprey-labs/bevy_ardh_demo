Generate textures for each face of cube map, very slow no parallelism or optimization
```
git clone https://github.com/birdsofprey-labs/bevy_rocky.git
cargo run --example ncube --release
```
clone and run following python script
these scripts split the big texture into smaller per tile textures
```
git clone https://github.com/birdsofprey-labs/bevy_rockpaperscissor.git
> rps_par.sh
```
here is the lib and demo, very unclean and hardcoded stuff like planet radius asssumed etc.
```
git clone https://github.com/birdsofprey-labs/bevy_ardh_demo.git
git clone https://github.com/birdsofprey-labs/bevy_ardh.git
```
copy assets in base folder
```
> copy_assets.sh
```
Ask my if something is not running.

Todo:
Lots of stuff
