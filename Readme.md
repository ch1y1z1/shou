# 手势跟踪机械手

## 预览

![](gif.gif)

## 编译

在编译之前，请先阅读[The Rust on ESP Book](https://esp-rs.github.io/book/)配置编译环境。

```bash
# to build the web frontend
cd shou-web
yarn && yarn build
# then mv the output "index.html" to "/shou/static/", it's already put in the right place

# to build the esp-rust part
# before build, you may modify the wifi ssid and password to your own
cd shou
. ~/export-esp.sh
cargo run --release
# then in the output (serial) ,there will be info which gives esp's static ip
# open the ip directly, you will see a simple frontend to control the hand

# to run the detect and sync part
# before this part, you may modify the url to your own ip from last part
cd shou-detect
pip install -r requirements.txt
python main.py
```