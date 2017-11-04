<!-- # @todo put referral URL here -->

🛫 Fastest way to expose a RSS feed to your [LaMetric](https://developer.lametric.com/) 🎩.

![lametric-app](/docs/lametric-app.jpg)

# Setup

```bash
cargo install rss-to-lametric
rss-to-lametric
```


# 🤓 Documentation

```bash
curl --silent http://localhost:8000/
```

```json
{
  "description": "Rss feed to LaMetric app",
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

# 🎩 Convert any RSS to LaMetric `TextFrame`

```bash
curl --silent "http://localhost:8000/convert?title=Ouest-France&icon=i14532&limit=3&url=https://www.ouest-france.fr/rss-en-continu.xml"
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

# ⚙️ Deployment 
- Deploy it (e.g. on [Clever-cloud](https://clever-cloud.com))
- [Create and configure your app](https://developer.lametric.com/)
- Done!
