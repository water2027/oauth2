第三方服务
  - 注册服务(登记redirect_uri, 获取client_id, client_secret)
  - 换取token(code换取双token) POST /token
  - 刷新token
  - token兑换信息
  - 配置信息(redirect_uri)
  - 作废refresh_token POST /revoke
  - 发现文档和JWKS GET /.well-known/openid-configuration GET /jwks
  
用户
  - 修改密码
  - 注册
  - 登录
  - 发送邮件验证码
  - 同意/拒绝授权(验证参数, 生成code并重定向) GET /authorize + POST /authorize
  - 修改个人信息
  - 授权管理
  - 成为开发者
  
Client侧提供通过xx登录. 

用户点击通过xx登录, 跳转至页面(GET /authorize?redirect_uri=xxx&client_id=xxx&scope=xxx)

xx侧生成一个auth_id, 一方面作为csrf_token, 另一方面与redirect_uri, client_id, scope等成映射关系, 返回一个页面

如果没登录, 先登录. 

如果已经在xx登录过了, 点击同意授权后进行授权(POST /authorize)

根据表单传递的auth_id, 找出对应的数据, 生成授权码, 让用户重定向至redirect_uri?code=xxx

Client侧通过secret和code换取双token, 之后使用access_token进行用户信息的获取