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