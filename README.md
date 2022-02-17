# service_ctl
service controller


## Todo
- RaftMessage / Message 的格式需要重新定义
- PollContext?
- RaftRouter 用来转发消息
- Peer 里面处理 read 和 write 的 API
- RaftCmdRequst
- RaftLocalState
- PeerStorage 里面的 first_index 和 last_index
- Engine 拆分 or 不拆分
- 有关 raft 里面 snapshot 的 field 需要额外定义
- Entry 定义是否需要重新写
- proposalqueue (50%)