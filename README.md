# behemoth
[![Build Status](https://travis-ci.org/sdleffler/behemoth.svg?branch=master)](https://travis-ci.org/sdleffler/behemoth)

A horrible, demonic experiment in Rust macro abuse.

Because I never stopped to ask "should I, really?" when I realized that all of my problems could be solved by the overzealous application of macros everywhere.

Oh no, I need true higher kinded types! Oh no, I need to implement various specific multiplication functions between matrices! Oh, nevermind, I'll just use this massive O(n^2) macro.

Oh crap! This will be horribly slow at runtime! NEVER FEAR! We'll just make 64 special cases, and the slow general case will be fine on its own.

The Behemoth linear algebra library promises to add up to 20 seconds on to your compile time! It features the ability to define matrix types, vector types, and more on demand - simply add another line to the appropriate macro! <sub>Regret, sorrow, and irritation are common side effects of using the Behemoth linear algebra library. If you begin to experience chest pain while using the Behemoth linear algebra library, please visit a doctor.</sub>

Behemoth<sup>TM</sup> - because pain at compile-time is always preferable ~~to pain at run-time~~. Recommended by masochists everywhere!

This project is so nasty that integral parts of it depend on a fix for a Rust ICE that I just put in a pull request for, which is why the Travis build is currently failing.
