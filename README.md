# SVG Icon Helper for [yew framework](https://github.com/DenisKolodin/yew) 

## USAGE

### 1) with remote svg-defs:
```
html!{
    <SVG: href="/svg-defs.svg#shape",/>
}
```
### 2) with local *.svg file or string:
```
html!{
    <SVG: content=include_str!("../svgs/satellite.svg"),/>
}
```
