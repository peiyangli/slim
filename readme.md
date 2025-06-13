### slint测试项目

1. slint文档： https://docs.slint.dev/latest/docs/slint/
2. slint项目： https://github.com/slint-ui/slint
3. 组件参考： https://github.com/Surrealism-All/SurrealismUI

### 基本的聊天消息页面
1. 无边框主页面
2. 联系人列表
3. 输入框
4. 消息列表

### 问题
1. 不支持富文本
2. ListView在元素高度不确定的情况，添加元素后设置viewport-y计算不准确
3. ListView没有定位到元素的API
4. ListView的滑动条不能单独定制风格

### 结论
1. 简单的工具类可以快速实现，并且很小无依赖。
2. 缺点是无法制作复杂的UI。