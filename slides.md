# Rust

#### Dan Callahan &middot; dcallahan@mozilla.com · [@callahad](https://twitter.com/callahad)

<br>

***

<!-- .slide: data-background="#1E1E21 radial-gradient(ellipse farthest-side at left top , #00549E 0px, transparent 100%) no-repeat scroll center top / 100% 500px" -->

![](img/firefox-developer_logo-wordmark_RGB.png)

### [firefox.com/developer](https://firefox.com/developer)

---

## C is software's Latin

***

### Memory management is **hard**.

<video data-autoplay class="stretch" src="img/ghostride.mp4"></video>

***

- Heartbleed
- Ghost <!-- .element: class="fragment" -->
- CVE-2015-0080 <!-- .element: class="fragment" -->

_I'm not smarter than the glibc or openssl devs._
<!-- .element: class="fragment" -->

---

### Rust

<br>

<span class="fragment">C's Performance</span><span class="fragment">, Portability</span><span class="fragment">, and Embeddability.</span>

<!-- .element: class="fragment" --> With *guaranteed* safety.

***

> "We thought we would move some of our agent gem to C++, but I didn't feel
> comfortable maintaining a growing codebase written in an unsafe language that
> ran inside of our customers' applications.
>
> Before long, we shipped a version of the Ruby gem with parts written in Rust,
> and haven't looked back."

-- [@wycats](https://twitter.com/wycats), 23 Sep 2014

***

<!-- .slide: data-background-transition="none" data-background="img/jvns-whyrust.jpg" -->

&nbsp;

---

## Safety

***

<!-- .slide: data-background-transition="none" data-background="img/whiteboard/a01.jpg" -->

&nbsp;

***

<!-- .slide: data-background-transition="none" data-background="img/whiteboard/a02.jpg" -->

&nbsp;

***

<!-- .slide: data-background-transition="none" data-background="img/whiteboard/a03.jpg" -->

&nbsp;

***

<!-- .slide: data-background-transition="none" data-background="img/whiteboard/a04.jpg" -->

&nbsp;

***

<!-- .slide: data-background-transition="none" data-background="img/whiteboard/b01.jpg" -->

&nbsp;

***

<!-- .slide: data-background-transition="none" data-background="img/whiteboard/b02.jpg" -->

&nbsp;

***

<!-- .slide: data-background-transition="none" data-background="img/whiteboard/b03.jpg" -->

&nbsp;

***

<!-- .slide: data-background-transition="none" data-background="img/whiteboard/c01.jpg" -->

&nbsp;

***

<!-- .slide: data-background-transition="none" data-background="img/whiteboard/c02.jpg" -->

&nbsp;

***

<!-- .slide: data-background-transition="none" data-background="img/whiteboard/c03.jpg" -->

&nbsp;

***

<!-- .slide: data-background-transition="none" data-background="img/whiteboard/c04.jpg" -->

&nbsp;

***

<!-- .slide: data-background-transition="none" data-background="img/whiteboard/c05.jpg" -->

&nbsp;

***

<!-- .slide: data-background-transition="none" data-background="img/whiteboard/c06.jpg" -->

&nbsp;

***

<!-- .slide: data-background-transition="none" data-background="img/whiteboard/c07.jpg" -->

&nbsp;

***

<!-- .slide: data-background-transition="none" data-background="img/whiteboard/c08.jpg" -->

&nbsp;

***

<!-- .slide: data-background-transition="none" data-background="img/whiteboard/d01.jpg" -->

&nbsp;

***

<!-- .slide: data-background-transition="none" data-background="img/whiteboard/d02.jpg" -->

&nbsp;

***

<!-- .slide: data-background-transition="none" data-background="img/whiteboard/d03.jpg" -->

&nbsp;

***

<!-- .slide: data-background-transition="none" data-background="img/whiteboard/d04.jpg" -->

&nbsp;

***

<!-- .slide: data-background-transition="none" data-background="img/whiteboard/d05.jpg" -->

&nbsp;

***

<!-- .slide: data-background-transition="none" data-background="img/whiteboard/d06.jpg" -->

&nbsp;

***

<!-- .slide: data-background-transition="none" data-background="img/whiteboard/d07.jpg" -->

&nbsp;

***

<!-- .slide: data-background-transition="none" data-background="img/whiteboard/rules.jpg" -->

&nbsp;

***

### Enforced Statically at Compile Time

- If it compiles, it won't segfault.
- If it compiles, there are no data races.
- No cost at runtime.

***

> "Putting Dropbox's first Rust daemon into production, today!
>
> In a nutshell, if Dropbox loses any of your data in the future, you can safely assume Mozilla is to blame. :-P"

-- [@jamwt](https://twitter.com/jamwt), 15 Dec 2015

---

## Mutability Demos!

---

## C-compatible FFI!

---

## Servo

---

## Learn More

_[rust-lang.org](https//rust-lang.org)_

_[rustbyexample.com](http://rustbyexample.com/)_

_[reddit.com/r/rust](https://reddit.com/r/rust/)_

___

[@callahad](https://twitter.com/callahad)

dcallahan@mozilla.com
