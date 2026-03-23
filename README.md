<h1 align="center">Dockerize Rust</h1>

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

___

<p align="center">
  <a href="https://numq.github.io/support">
    <img src="https://api.qrserver.com/v1/create-qr-code/?size=112x112&data=https://numq.github.io/support&bgcolor=1a1b26&color=7aa2f7" 
         width="112" 
         height="112" 
         style="border-radius: 4px;" 
         alt="QR code">
  </a>
  <br>
  <a href="https://numq.github.io/support" style="text-decoration: none;">
    <code><font color="#bb9af7">numq.github.io/support</font></code>
  </a>
</p>
