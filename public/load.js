/**
 * Pressing the TAB key in a text area makes it lose focus and move to the next component, but
 * if we want people to write code, then they'll possibly use a TAB. So, let's enable it selectively.
 */
var et=function(f){var a=document.getElementById(f);a.onkeydown=function(b){if(9!=b.key||b.shiftKey||b.ctrlKey||b.altKey)return!0;var e=a.scrollTop;if(a.setSelectionRange){var c=a.selectionStart,g=a.selectionEnd;a.value=a.value.substring(0,c)+"\t"+a.value.substr(g);a.setSelectionRange(c+1,c+1);a.focus()}else a.createTextRange&&(document.selection.createRange().text="\t",b.returnValue=!1);a.scrollTop=e;b.prevDefault&&b.prevDefault();return!1}},TLN={evl:{},uln:function(f,a){var b=f.value.split("\n").length-
a.children.length;if(0<b){for(var e=document.createDocumentFragment();0<b;){var c=document.createElement("span");c.className="tln-line";e.appendChild(c);--b}a.appendChild(e)}for(;0>b;)a.removeChild(a.lastChild),b+=1},aln:function(f){var a=document.getElementById(f);if(null==a)return console.warn("Couldn't find '"+f+"'");var b=document.createElement("div");b.className="tln-wrapper";a.parentNode.insertBefore(b,a);TLN.uln(a,b);TLN.evl[f]=[];for(var e=["propertychange","input","keydown","keyup"],c=function(h,
k){return function(d){(10!=+h.scrollLeft||37!=d.keyCode&&37!=d.which&&"ArrowLeft"!=d.code&&"ArrowLeft"!=d.key)&&36!=d.keyCode&&36!=d.which&&"Home"!=d.code&&"Home"!=d.key&&13!=d.keyCode&&13!=d.which&&"Enter"!=d.code&&"Enter"!=d.key&&"NumpadEnter"!=d.code||(h.scrollLeft=0);TLN.uln(h,k)}}(a,b),g=e.length-1;0<=g;--g)a.addEventListener(e[g],c,{passive:!0}),TLN.evl[f].push({evt:e[g],hdlr:c});e=["change","mousewheel","scroll"];b=function(h,k){return function(d){return k.scrollTop=h.scrollTop}}(a,b);for(c=
e.length-1;0<=c;--c)a.addEventListener(e[c],b,{passive:!0}),TLN.evl[f].push({evt:e[c],hdlr:b})}};

// Lazy load some stuff
(async () => {
  const module = await import("./lce.js");
  await module.default();
  et("code-area");
  TLN.aln("code-area");
})();
