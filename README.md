# rust-todo
todo demo app by gotham

## 安装

默认你已经安装了 Rust，如果没有，请参考 [Install Rust](https://www.rust-lang.org/learn/get-started)

首先需要有一个 Postgres 数据库，如果没有可以用 Docker 快速安装

```bash
docker run -d --name pg-db -p 5432:5432 -E POSTGRES_ROOT_PASSWD=1234TttT postgres
```

接着安装 `diesel_cli`，完整文档参考官网 [Diesel - Getting Start](http://diesel.rs/guides/getting-started/)

```bash
# 需要安装 pg 客户端
sudo apt install postgresql-client
cargo install diesel_cli --no-default-features --features postgres
```

如果数据库不在本地运行或者修改了端口，请修改 `.env` 文件中的 `DATABASE_URL` 为新的地址、用户名和密码

接着在项目根目录下运行以下命令初始化数据库

```bash
diesel setup
diesel migration run
```

数据初始化完成后就可以运行项目

```bash
cargo run
```

访问 `http://localhost:8000` 即可看到页面
