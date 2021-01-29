# V8

Embed V8 into Ruby via [rusty_v8][1]


## Usage

``` ruby
require 'v8';

V8.eval('"Hello World"') #=> "Hello World"
```

## Development

``` text
$ bundle install
$ bundle exec rspec
```

[1]: https://github.com/denoland/rusty_v8
