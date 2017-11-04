# Your favorite RSS feed directly from your [LaMetric](http://bit.ly/2zcEaTz) 🎩

<p align="center">
  <a href="http://bit.ly/2zyD4SA">
  <img src="https://media.giphy.com/media/xT0xeG7WTqh4yQnnsk/giphy.gif"/>  
  </a>
</p>


## 📱 LaMetric apps

### French news

- 🇫🇷 [Ouest-France 👌](http://bit.ly/2zyD4SA)
- 🇫🇷 [Actu.fr](http://bit.ly/2lQ7TfO)
- 🇫🇷 [Presse Ocean](http://bit.ly/2zxGZiN)
- 🇫🇷 [20 minutes](http://bit.ly/2zjSrO8)
- 🇫🇷 [Voiles Et Voiliers](http://bit.ly/2lQ2Bkm)
- 🇫🇷 [Courrier de l'Ouest](http://bit.ly/2lRj5so)

### World news

- 🇺🇸 [New York Times](http://bit.ly/2yuBMYO)
- 🇺🇸 [Washington Post](http://bit.ly/2zdXQq8)
- 🇬🇧 [The Guardian](http://bit.ly/2y0bXvA)
- 🇩🇪 [Der Spiegel](http://bit.ly/2hb0ia1)

### Tech/Startup

- 🇫🇷 [BFM Business](http://bit.ly/2zeGdGZ)
- 🇺🇸 [Tech Crunch](http://bit.ly/2ze84qM)
- 🇺🇸 [Mashable](http://bit.ly/2hc9fQn)
- 🇺🇸 [Lifehacker](http://bit.ly/2xZbc5H)
- 🇺🇸 [Engadget](http://bit.ly/2lRyRTZ)


## 👉 Try it online

[https://rss-to-lametric.cleverapps.io/convert?title=Ouest-France&icon=i14532&limit=3&url=https://www.ouest-france.fr/rss-en-continu.xml](http://rss-to-lametric.cleverapps.io/convert?title=Ouest-France&icon=i14532&limit=3&url=https://www.ouest-france.fr/rss-en-continu.xml)



[![Travis](https://img.shields.io/travis/rust-lang/rust.svg)](https://travis-ci.org/FGRibreau/rss-to-lametric) [![codecov](https://codecov.io/gh/FGRibreau/rss-to-lametric/branch/master/graph/badge.svg)](https://codecov.io/gh/FGRibreau/rss-to-lametric)
[![Cargo version](https://img.shields.io/crates/v/rss-to-lametric.svg)](https://crates.io/crates/rss-to-lametric) [![Crates.io](https://img.shields.io/crates/l/rss-to-lametric.svg)](https://crates.io/crates/rss-to-lametric) [![Crates.io](https://img.shields.io/crates/d/rss-to-lametric.svg)](https://crates.io/crates/rss-to-lametric)


<p align="center">
  <a href="http://bit.ly/2zyD4SA">
  <img src="/docs/lametric-app.jpg"/>  
  </a>
</p>

## ⏲ Setup

```bash
cargo install rss-to-lametric
```


## 🤓 Self-documentation

```bash
rss-to-lametric
curl --silent http://localhost:8080/
```

```json
{
  "description": "🛫 Fastest way to expose a RSS feed to your LaMetric - http://bit.ly/2zcEaTz 🎩",
  "homepage": "https://github.com/FGRibreau/rss-to-lametric",
  "name": "rss-to-lametric",
  "usage": [
    {
      "path": "/convert?<query>",
      "query": {
        "icon": "icon number, e.g. i14532",
        "limit": 10,
        "title": "title of your feed",
        "url": "http://my-domain.com/my-rss.xml"
      }
    }
  ],
  "version": "0.1.0"
}
```

## 🎩 Convert any RSS to LaMetric `TextFrame`

```bash
curl --silent "http://localhost:8080/convert?title=Ouest-France&icon=i14532&limit=3&url=https://www.ouest-france.fr/rss-en-continu.xml"
```

```json
{
  "frames": [
    {
      "TextFrame": {
        "text": "Ouest-France",
        "icon": "i14532"
      }
    },
    {
      "TextFrame": {
        "text": "Dopage. Le laboratoire français suspendu jusqu'à six mois",
        "icon": null
      }
    },
    {
      "TextFrame": {
        "text": "Stade Rennais. Olivier Létang nommé président délégué et manager général",
        "icon": null
      }
    },
    {
      "TextFrame": {
        "text": "Stade Rennais. Le président du club René Ruello annonce sa démission",
        "icon": null
      }
    }
  ]
}
```

## ⚙️ Deployment 
- Deploy it (the fastest way is to use [Clever-cloud](https://www.clever-cloud.com/doc/rust/rust/) thanks to their awesome native Rust support)
- [Create and configure your app](http://bit.ly/2hcJobb)
- Done!
