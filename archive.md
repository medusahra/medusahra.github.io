---
layout: default
title: Archive
---
[< volver](/)

# Archive

{% assign pinned_posts = site.posts | where: "pinned", true %}
{% assign normal_posts = site.posts | where_exp: "post", "post.pinned != true" %}

{% for post in pinned_posts %}
- {{ post.date | date: "%Y-%m-%d" }} <span class="pin-icon">📌</span> [{{ post.title }}]({{ post.url }})
{% endfor %}

{% for post in normal_posts %}
- {{ post.date | date: "%Y-%m-%d" }} [{{ post.title }}]({{ post.url }})
{% endfor %}

<style>
.pin-icon {
  color: #ff1493;
  text-shadow: 0 0 10px #ff1493, 0 0 20px #ff1493;
  animation: pulse-glow 2s ease-in-out infinite;
  display: inline-block;
}

@keyframes pulse-glow {
  0%, 100% { 
    opacity: 1;
    text-shadow: 0 0 10px #ff1493, 0 0 20px #ff1493;
  }
  50% { 
    opacity: 0.7;
    text-shadow: 0 0 5px #ff1493, 0 0 10px #ff1493;
  }
}
</style>
