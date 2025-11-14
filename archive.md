---
layout: default
title: Archive
---

[< volver](/)

# Archive

{% assign pinned_posts = site.posts | where: "pinned", true %}
{% assign normal_posts = site.posts | where_exp: "post", "post.pinned != true" %}

{% for post in pinned_posts %}
- {{ post.date | date: "%Y-%m-%d" }} ⭐ [{{ post.title }}]({{ post.url }})
{% endfor %}

{% for post in normal_posts %}
- {{ post.date | date: "%Y-%m-%d" }} [{{ post.title }}]({{ post.url }})
{% endfor %}
