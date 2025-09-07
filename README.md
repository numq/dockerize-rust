<h1 align="center">Dockerize Rust</h1>

<br>

<div align="center" style="display: grid; justify-content: center;">

|                                                                  🌟                                                                   |                  Support this project                   |               
|:-------------------------------------------------------------------------------------------------------------------------------------:|:-------------------------------------------------------:|
|  <img src="https://raw.githubusercontent.com/ErikThiart/cryptocurrency-icons/master/32/bitcoin.png" alt="Bitcoin (BTC)" width="32"/>  | <code>bc1qs6qq0fkqqhp4whwq8u8zc5egprakvqxewr5pmx</code> | 
| <img src="https://raw.githubusercontent.com/ErikThiart/cryptocurrency-icons/master/32/ethereum.png" alt="Ethereum (ETH)" width="32"/> | <code>0x3147bEE3179Df0f6a0852044BFe3C59086072e12</code> |
|  <img src="https://raw.githubusercontent.com/ErikThiart/cryptocurrency-icons/master/32/tether.png" alt="USDT (TRC-20)" width="32"/>   |     <code>TKznmR65yhPt5qmYCML4tNSWFeeUkgYSEV</code>     |

</div>

<br>

<p align="center">Docker Multi-stage Build for Tiny Rust Image Deployment based on linux musl</p>

*[Multi-stage](https://docs.docker.com/build/building/multi-stage/) - *this is a way to reduce an image size creating an empty image and copying the binary file into it.*

<ul>
    <li>First stage</li>
    <ul>
        <li>Create a new image</li>
        <li>Install dependencies</li>
        <li>Build binaries</li>
    </ul>
    <li>Second stage</li>
    <ul>
        <li>Create a new image</li>
        <li>Copy binaries</li>
        <li>Launch</li>
    </ul>
</ul>

# Run:
```
docker compose up [OPTIONS] [SERVICE...]
```
