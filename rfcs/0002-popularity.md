# Popularity (声望)
- 功能: (唯一标识, `my_awesome_feature`)
- 创建日期: (rfc的创建时间, YYYY-MM-DD)


## 概括

应用内创作能力的体现。

## 动机

我们为什么这样做呢？ 它支持哪些用例？ 预期的结果是什么？

## 介绍

声望的数据定义如下:

    * id --数据库索引 int32
    * user_id 数据库 id
    * level 声望等级 int32
    * name 当前声望的称号 vchar20
    * count 当前经验 Int32
    * create_at 创建时间 time
    * update_at 更新时间 time
    * level_template_name 使用模板名称


## 参考

省略

## 缺点

省略

## 基本原理和替代方案

省略

## 现有技术


省略

## 未被解决的问题

省略

## 展望未来

省略