# service_ctl
service controller


## Todo

### 需要修改
- `Entry`
- `Peer_id`的作用
- `eraftpb`
- ......

- 需要存什么？
- ChangeLog: get(), put() -----  定义 `key`的形式
- ClusterMap： 
    - get_latest() / update() 是通过 apply changelog 来实现的
    - 改变`target`的状态的API， 生成新的 changelog， 调用put()
- checkliveness?
- config 部分， 提供 get/put 就够了
- oid 生成API
- `storage`模块测试代码
- ......
