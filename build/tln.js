export const TLN={eventList:{},update_line_numbers:async(e,t)=>{let n=e.value.split("\n").length-t.children.length;if(n>0){const e=document.createDocumentFragment();for(;n>0;){const t=document.createElement("span");(t.className="tln-line"),e.appendChild(t),n-=1}t.appendChild(e)}for(;n<0;){t.removeChild(t.lastChild),n+=1}},append_line_numbers:async(e)=>{const t=document.getElementById(e);if(null==t){return console.warn("[tln.js] Couldn't find textarea of id '"+e+"'")}const n=document.createElement("div");(n.className="tln-wrapper"),t.parentNode.insertBefore(n,t),TLN.update_line_numbers(t,n),(TLN.eventList[e]=[]);const l=["propertychange","input","keydown","keyup"],o=(function(e,t){return function(n){((10!= +e.scrollLeft||(37!=n.keyCode&&37!=n.which&&"ArrowLeft"!=n.code&&"ArrowLeft"!=n.key))&&36!=n.keyCode&&36!=n.which&&"Home"!=n.code&&"Home"!=n.key&&13!=n.keyCode&&13!=n.which&&"Enter"!=n.code&&"Enter"!=n.key&&"NumpadEnter"!=n.code)||(e.scrollLeft=0),TLN.update_line_numbers(e,t)}})(t,n);for(let n=l.length-1;n>=0;n-=1){t.addEventListener(l[n],o,{passive:!0}),TLN.eventList[e].push({evt:l[n],hdlr:o})}const r=["change","mousewheel","scroll"],s=(function(e,t){return function(){t.scrollTop=e.scrollTop}})(t,n);for(let n=r.length-1;n>=0;n-=1){t.addEventListener(r[n],s,{passive:!0}),TLN.eventList[e].push({evt:r[n],hdlr:s})}}};