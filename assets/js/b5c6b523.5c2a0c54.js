"use strict";(self.webpackChunkmy_website=self.webpackChunkmy_website||[]).push([[1613],{172:(e,n,t)=>{t.r(n),t.d(n,{assets:()=>s,contentTitle:()=>r,default:()=>l,frontMatter:()=>a,metadata:()=>o,toc:()=>c});var i=t(5893),A=t(1151);const a={},r="\u5165\u9580",o={id:"3d/0",title:"\u5165\u9580",description:"\u4f7f\u7528 image \u4f5c\u70ba\u5716\u50cf\u5eab\u3002",source:"@site/coding/3d/0.md",sourceDirName:"3d",slug:"/3d/0",permalink:"/blog/coding/3d/0",draft:!1,unlisted:!1,tags:[],version:"current",frontMatter:{},sidebar:"tutorialSidebar",previous:{title:"\u7c21\u4ecb",permalink:"/blog/coding/3d/"},next:{title:"1. \u756b\u76f4\u7dda",permalink:"/blog/coding/3d/1"}},s={},c=[];function d(e){const n={code:"code",h1:"h1",img:"img",p:"p",pre:"pre",...(0,A.a)(),...e.components};return(0,i.jsxs)(i.Fragment,{children:[(0,i.jsx)(n.h1,{id:"\u5165\u9580",children:"\u5165\u9580"}),"\n",(0,i.jsxs)(n.p,{children:["\u4f7f\u7528 ",(0,i.jsx)(n.code,{children:"image"})," \u4f5c\u70ba\u5716\u50cf\u5eab\u3002"]}),"\n",(0,i.jsx)(n.pre,{children:(0,i.jsx)(n.code,{children:"$ cargo add image\n"})}),"\n",(0,i.jsx)(n.pre,{children:(0,i.jsx)(n.code,{className:"language-rust",children:'use image::imageops::flip_vertical_in_place;\n\nconst RED: image::Rgba<u8> = image::Rgba([255, 0, 0, 255]);\n\nfn main() {\n    // Create a new ImgBuf with width: imgx and height: imgy\n    let mut imgbuf = image::ImageBuffer::new(100, 100);\n\n    imgbuf.put_pixel(52, 41, RED);\n    flip_vertical_in_place(&mut imgbuf);\n\n    imgbuf.save("output.tga").unwrap();\n}\n'})}),"\n",(0,i.jsx)(n.p,{children:"\u4fdd\u5b58\u7684\u5716\u50cf\u683c\u5f0f\u70ba TGA\uff0c\u53ef\u4ee5\u7528 Photoshop \u6253\u958b\u3002"}),"\n",(0,i.jsx)(n.p,{children:(0,i.jsx)(n.img,{alt:"Alt text",src:t(8667).Z+"",width:"100",height:"100"})})]})}function l(e={}){const{wrapper:n}={...(0,A.a)(),...e.components};return n?(0,i.jsx)(n,{...e,children:(0,i.jsx)(d,{...e})}):d(e)}},8667:(e,n,t)=>{t.d(n,{Z:()=>i});const i="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAGQAAABkCAIAAAD/gAIDAAAACXBIWXMAAAsTAAALEwEAmpwYAAAAO0lEQVR4nO3OMQ0AAAgDsPk3DSaWcNAqaAIAAAAAAAAAAAAAAAAAAAAAUDHXAQAAAAAAAAAAAAAA4IcFDK4BAIF1Z0UAAAAASUVORK5CYII="},1151:(e,n,t)=>{t.d(n,{Z:()=>o,a:()=>r});var i=t(7294);const A={},a=i.createContext(A);function r(e){const n=i.useContext(a);return i.useMemo((function(){return"function"==typeof e?e(n):{...n,...e}}),[n,e])}function o(e){let n;return n=e.disableParentContext?"function"==typeof e.components?e.components(A):e.components||A:r(e.components),i.createElement(a.Provider,{value:n},e.children)}}}]);