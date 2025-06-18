---
layout: default
title: Home
---
<section>
  <h1>Blog Posts</h1>
  <ul>
    {% for post in site.posts %}
      <li>
        <a href="{{ post.url }}">{{ post.title }}</a><br>
        <small>{{ post.date | date: "%B %-d, %Y" }}</small>
      </li>
    {% endfor %}
  </ul>
</section>
