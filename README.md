# Table-Structure-Generator

## How to use

```
USAGE:
    table-structure-generator [OPTIONS] --password <PASSWORD> [TABLES]...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --database <DATABASE>    Sets database
        --host <HOST>            Sets the host
        --password <PASSWORD>    Sets the password
    -p, --port <PORT>            Sets the port
    -U, --username <USERNAME>    Sets PG username

ARGS:
    <TABLES>...
```

This will print the html to `stdin` .

>  Tips:**You can use `>` to redirect the output to a certain file **

## Example Preview

<h3>location</h3><table border="1"><thead><tr><th>字段名称</th><th>数据类型</th><th>是否可空</th><th>默认值</th></tr></thead><tbody><tr><td>location_id</td><td>integer</td><td>NO</td><td>Null</td></tr><tr><td>location_name</td><td>character varying</td><td>YES</td><td>Null</td></tr><tr><td>dept_id</td><td>integer</td><td>YES</td><td>Null</td></tr></tbody></table><h3>user_info</h3><table border="1"><thead><tr><th>字段名称</th><th>数据类型</th><th>是否可空</th><th>默认值</th></tr></thead><tbody><tr><td>user_id</td><td>integer</td><td>NO</td><td>Null</td></tr><tr><td>permission</td><td>USER-DEFINED</td><td>YES</td><td>Null</td></tr><tr><td>score</td><td>integer</td><td>YES</td><td>Null</td></tr><tr><td>user_name</td><td>character varying</td><td>YES</td><td>Null</td></tr><tr><td>user_password</td><td>character varying</td><td>YES</td><td>Null</td></tr><tr><td>dept_id</td><td>integer</td><td>YES</td><td>Null</td></tr><tr><td>avatar</td><td>bytea</td><td>YES</td><td>Null</td></tr></tbody></table><h3>score_history</h3><table border="1"><thead><tr><th>字段名称</th><th>数据类型</th><th>是否可空</th><th>默认值</th></tr></thead><tbody><tr><td>history_id</td><td>integer</td><td>NO</td><td>Null</td></tr><tr><td>now_score</td><td>integer</td><td>YES</td><td>Null</td></tr><tr><td>prev_score</td><td>integer</td><td>YES</td><td>Null</td></tr><tr><td>user_id</td><td>integer</td><td>YES</td><td>Null</td></tr><tr><td>history_time</td><td>timestamp without time zone</td><td>YES</td><td>Null</td></tr></tbody></table><h3>take</h3><table border="1"><thead><tr><th>字段名称</th><th>数据类型</th><th>是否可空</th><th>默认值</th></tr></thead><tbody><tr><td>take_id</td><td>integer</td><td>NO</td><td>Null</td></tr><tr><td>take_status</td><td>USER-DEFINED</td><td>YES</td><td>Null</td></tr><tr><td>course_id</td><td>integer</td><td>YES</td><td>Null</td></tr><tr><td>take_time</td><td>timestamp without time zone</td><td>YES</td><td>Null</td></tr><tr><td>team_num</td><td>integer</td><td>YES</td><td>Null</td></tr></tbody></table><h3>friendship</h3><table border="1"><thead><tr><th>字段名称</th><th>数据类型</th><th>是否可空</th><th>默认值</th></tr></thead><tbody><tr><td>user_to_id</td><td>integer</td><td>NO</td><td>Null</td></tr><tr><td>user_from_id</td><td>integer</td><td>NO</td><td>Null</td></tr></tbody></table><h3>department</h3><table border="1"><thead><tr><th>字段名称</th><th>数据类型</th><th>是否可空</th><th>默认值</th></tr></thead><tbody><tr><td>dept_id</td><td>integer</td><td>NO</td><td>Null</td></tr><tr><td>dept_name</td><td>character varying</td><td>YES</td><td>Null</td></tr></tbody></table><h3>take_user_info</h3><table border="1"><thead><tr><th>字段名称</th><th>数据类型</th><th>是否可空</th><th>默认值</th></tr></thead><tbody><tr><td>take_id</td><td>integer</td><td>NO</td><td>Null</td></tr><tr><td>user_id</td><td>integer</td><td>NO</td><td>Null</td></tr></tbody></table><h3>course</h3><table border="1"><thead><tr><th>字段名称</th><th>数据类型</th><th>是否可空</th><th>默认值</th></tr></thead><tbody><tr><td>course_id</td><td>integer</td><td>NO</td><td>Null</td></tr><tr><td>capacity</td><td>integer</td><td>YES</td><td>Null</td></tr><tr><td>course_name</td><td>character varying</td><td>YES</td><td>Null</td></tr><tr><td>end_time</td><td>timestamp without time zone</td><td>YES</td><td>Null</td></tr><tr><td>num</td><td>integer</td><td>YES</td><td>Null</td></tr><tr><td>start_time</td><td>timestamp without time zone</td><td>YES</td><td>Null</td></tr><tr><td>visible</td><td>boolean</td><td>YES</td><td>Null</td></tr><tr><td>dept_id</td><td>integer</td><td>YES</td><td>Null</td></tr><tr><td>location_id</td><td>integer</td><td>YES</td><td>Null</td></tr></tbody></table>

