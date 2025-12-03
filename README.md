# awesome_env

**Zero-dependency, blazingly fast `.env` file loader for Rust**  
Không cần `dotenvy`, không cần `std::env::set_var`, chỉ cần một dòng là load xong!

[![Crates.io](https://img.shields.io/crates/v/awesome_env.svg)](https://crates.io/crates/awesome_env)
[![Downloads](https://img.shields.io/crates/d/awesome_env.svg)](https://crates.io/crates/awesome_env)
[![License](https://img.shields.io/crates/l/awesome_env.svg)](https://crates.io/crates/awesome_env)
[![Docs](https://docs.rs/awesome_env/badge.svg)](https://docs.rs/awesome_env)

## Tại sao nên dùng `awesome_env`?

| Tính năng                       | awesome_env       | dotenvy crate               |
| ------------------------------- | ----------------- | --------------------------- |
| Zero dependencies               | Yes               | No                          |
| Hỗ trợ comment `#`              | Yes               | Yes                         |
| Hỗ trợ giá trị chứa `=`         | Yes               | Yes                         |
| Trim khoảng trắng tự động       | Yes               | Yes                         |
| Không tự động ghi đè `std::env` | Yes (an toàn hơn) | No (có thể gây side-effect) |
| Dễ test, dễ kiểm soát           | Yes               | Yes                         |
| Tốc độ (blazingly fast)         | Yes               | Yes                         |

## Cài đặt

```bash
cargo add awesome_env
```
