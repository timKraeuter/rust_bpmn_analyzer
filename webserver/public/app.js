import{n as e}from"./rolldown-runtime.js";import{Bn as t,Ct as n,Rt as r,Wt as i,_n as a,an as o,fn as s,hn as c,in as l,ln as u,mn as d,pn as f,sn as p,un as m}from"./diagram-vendor.js";import{a as h,i as g,n as ee,r as _,t as te}from"./bpmn-vendor.js";import{t as v}from"./vendor.js";(function(){let e=document.createElement(`link`).relList;if(e&&e.supports&&e.supports(`modulepreload`))return;for(let e of document.querySelectorAll(`link[rel="modulepreload"]`))n(e);new MutationObserver(e=>{for(let t of e)if(t.type===`childList`)for(let e of t.addedNodes)e.tagName===`LINK`&&e.rel===`modulepreload`&&n(e)}).observe(document,{childList:!0,subtree:!0});function t(e){let t={};return e.integrity&&(t.integrity=e.integrity),e.referrerPolicy&&(t.referrerPolicy=e.referrerPolicy),e.crossOrigin===`use-credentials`?t.credentials=`include`:e.crossOrigin===`anonymous`?t.credentials=`omit`:t.credentials=`same-origin`,t}function n(e){if(e.ep)return;e.ep=!0;let n=t(e);fetch(e.href,n)}})();var ne=`<?xml version="1.0" encoding="UTF-8"?>
<definitions xmlns:bpmndi="http://www.omg.org/spec/BPMN/20100524/DI" xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL" id="definitions_793d1020-e08a-4f53-b128-c61827f033c9" targetNamespace="http://www.omg.org/spec/BPMN/20100524/MODEL" exporter="Camunda Modeler" exporterVersion="5.7.0">
  <process id="process_c648aa44-f99b-4cc0-8bba-9b1fafd7d01b" isExecutable="false" />
  <bpmndi:BPMNDiagram id="BPMNDiagram_40e0d24a-67a5-413e-aefc-dad265aaf73b">
    <bpmndi:BPMNPlane id="BPMNPlane_500f5dcb-d27a-4260-8cac-962e6eb35e01" bpmnElement="process_c648aa44-f99b-4cc0-8bba-9b1fafd7d01b" />
  </bpmndi:BPMNDiagram>
</definitions>
`;function re(e){let t=ae(e,E.__wbindgen_malloc,E.__wbindgen_realloc),n=T,r=E.check_bpmn(t,n);if(r[2])throw oe(r[1]);return oe(r[0])}function ie(){let e={__proto__:null,__wbg_Error_83742b46f01ce22d:function(e,t){return Error(y(e,t))},__wbg___wbindgen_is_string_7ef6b97b02428fae:function(e){return typeof e==`string`},__wbg___wbindgen_throw_6ddd609b62940d55:function(e,t){throw Error(y(e,t))},__wbg_new_49d5571bd3f0c4d4:function(){return new Map},__wbg_new_a70fbab9066b301f:function(){return[]},__wbg_new_ab79df5bd7c26067:function(){return{}},__wbg_set_282384002438957f:function(e,t,n){e[t>>>0]=n},__wbg_set_6be42768c690e380:function(e,t,n){e[t]=n},__wbg_set_bf7251625df30a02:function(e,t,n){return e.set(t,n)},__wbindgen_cast_0000000000000001:function(e){return e},__wbindgen_cast_0000000000000002:function(e,t){return y(e,t)},__wbindgen_init_externref_table:function(){let e=E.__wbindgen_externrefs,t=e.grow(4);e.set(0,void 0),e.set(t+0,void 0),e.set(t+1,null),e.set(t+2,!0),e.set(t+3,!1)}};return{__proto__:null,"./rust_bpmn_analyzer_wasm_bg.js":e}}function y(e,t){return e>>>=0,ce(e,t)}var b=null;function x(){return(b===null||b.byteLength===0)&&(b=new Uint8Array(E.memory.buffer)),b}function ae(e,t,n){if(n===void 0){let n=w.encode(e),r=t(n.length,1)>>>0;return x().subarray(r,r+n.length).set(n),T=n.length,r}let r=e.length,i=t(r,1)>>>0,a=x(),o=0;for(;o<r;o++){let t=e.charCodeAt(o);if(t>127)break;a[i+o]=t}if(o!==r){o!==0&&(e=e.slice(o)),i=n(i,r,r=o+e.length*3,1)>>>0;let t=x().subarray(i+o,i+r),a=w.encodeInto(e,t);o+=a.written,i=n(i,r,o,1)>>>0}return T=o,i}function oe(e){let t=E.__wbindgen_externrefs.get(e);return E.__externref_table_dealloc(e),t}var S=new TextDecoder(`utf-8`,{ignoreBOM:!0,fatal:!0});S.decode();var se=2146435072,C=0;function ce(e,t){return C+=t,C>=se&&(S=new TextDecoder(`utf-8`,{ignoreBOM:!0,fatal:!0}),S.decode(),C=t),S.decode(x().subarray(e,e+t))}var w=new TextEncoder;`encodeInto`in w||(w.encodeInto=function(e,t){let n=w.encode(e);return t.set(n),{read:e.length,written:n.length}});var T=0,E;function le(e,t){return E=e.exports,b=null,E.__wbindgen_start(),E}async function ue(e,t){if(typeof Response==`function`&&e instanceof Response){if(typeof WebAssembly.instantiateStreaming==`function`)try{return await WebAssembly.instantiateStreaming(e,t)}catch(t){if(e.ok&&n(e.type)&&e.headers.get(`Content-Type`)!==`application/wasm`)console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",t);else throw t}let r=await e.arrayBuffer();return await WebAssembly.instantiate(r,t)}else{let n=await WebAssembly.instantiate(e,t);return n instanceof WebAssembly.Instance?{instance:n,module:e}:n}function n(e){switch(e){case`basic`:case`cors`:case`default`:return!0}return!1}}async function de(e){if(E!==void 0)return E;e!==void 0&&(Object.getPrototypeOf(e)===Object.prototype?{module_or_path:e}=e:console.warn(`using deprecated parameters for the initialization function; pass a single object instead`)),e===void 0&&(e=new URL(`/rust_bpmn_analyzer_wasm_bg.wasm`,``+import.meta.url));let t=ie();(typeof e==`string`||typeof Request==`function`&&e instanceof Request||typeof URL==`function`&&e instanceof URL)&&(e=fetch(e));let{instance:n,module:r}=await ue(await e,t);return le(n,r)}function fe(e){e.on(`analysis.start`,t=>{pe(t,e)})}function pe(e,t){de().then(()=>{let n=performance.now(),r=re(e.xml),i=performance.now()-n;console.log(`BPMN analysis runtime (wasm): `+i+` ms`),t.fire(`analysis.done`,r)}).catch(e=>{console.error(`WASM initialization failed:`,e)})}fe.$inject=[`eventBus`];var me={__init__:[`wasmAnalysis`],wasmAnalysis:[`type`,fe]},D=`counterexample.visualization.toggleMode`,O=`counterexample.playExecution`,k=`counterexample.pauseExecution`,he=`counterexample.animationCreated`,A=`counterexample.animationSpeedChanged`,j=`counterexample.execution.trace`,ge=`counterexample.visualization.start`,M=`counterexample.visualization.restart`,_e=`counterexample.visualization.started`,ve=`Safeness`,ye=`OptionToComplete`,be=`ProperCompletion`,xe=`NoDeadActivities`,Se=[ve,ye,be,xe],Ce=`analysis-note`;function we(e,t){e.on(`analysis.done`,n);function n(e){if(t.remove({type:Ce}),!(!e||!e.property_results))for(let t of e.property_results)t.property===`Safeness`&&!t.fulfilled&&r(t),t.property===`ProperCompletion`&&!t.fulfilled&&i(t),t.property===`NoDeadActivities`&&!t.fulfilled&&a(t),t.property===`OptionToComplete`&&!t.fulfilled&&o(t)}function r(e){s(e,{bottom:-5,left:5},`Two or more tokens`,`small-note`)}function i(e){s(e,{bottom:50,right:-5},`Consumes two or more tokens`,`big-note`)}function a(e){for(let t of e.problematic_elements)c(t,{bottom:-5,left:17.5},`Dead Activity`,`big-note`,!1)}function o(e){for(let n of e.problematic_elements)t.remove({element:n});s(e,{bottom:-5,left:5},`Flow can contain more than 50 tokens.`,`big-note`)}function s(t,n,r,i){for(let a of t.problematic_elements){c(a,n,r,i,!0);let o=document.getElementById(a+`_counter`);o&&o.addEventListener(`click`,()=>{e.fire(ge,{propertyResult:t})})}}function c(e,n,r,i,a){t.add(e,Ce,{position:n,html:`<div id="${e}_counter" class="property-note tooltip ${i}${a?` clickable`:``}">
               ${r}
               ${a?`<span class="tooltipText">Click to visualize an execution example.</span>`:``}
             </div>`})}}we.$inject=[`eventBus`,`overlays`];var Te={__init__:[`analysisOverlays`],analysisOverlays:[`type`,we]},N=75;function Ee(e,t){this.preExecute=function(n){let i=n.unsafeCause,a=e.createShape({type:`bpmn:ExclusiveGateway`},{x:i.x+i.width+N,y:r(i).y},i.parent),o=De(i,[]);t.makeSpace(o,[],{x:N,y:0},`e`,0);let s=i.outgoing.map(e=>e);for(let t of s)e.reconnectStart(t,a,r(a));e.connect(i,a)}}Ee.$inject=[`modeling`,`spaceTool`];function De(e,t){return e.outgoing.forEach(n=>{let r=n.target;r.x>e.x&&!t.includes(r)&&(t.push(r),r.label&&t.push(r.label),De(r,t))}),t}function Oe(e,t,n,i){let a=[],o=n.createShape({type:`bpmn:ExclusiveGateway`});o.x=e.x+e.width+N-o.width/2,o.y=r(e).y-o.height/2,a.push(o);let s=n.createConnection({type:`bpmn:SequenceFlow`});s.waypoints=i.layoutConnection(s,{source:e,target:o}),a.push(s),e.outgoing.forEach(e=>{let t=n.createConnection({type:`bpmn:SequenceFlow`}),s=n.createShape({type:e.target.type});s.x=e.target.x+N,s.y=e.target.y;let c=r(o);t.waypoints=i.layoutConnection(t,{source:o,target:s,connectionStart:c}),a.push(t)});let c={x:N,y:0},l=De(e,[]);l.forEach(e=>e.outgoing.forEach(e=>l.push(e)));let u=l.map(e=>({element:e,delta:c}));t.create({created:a,moved:u,removed:e.outgoing})}var P=-75;function ke(e,t){this.preExecute=function(n){let i=n.unsafeMerge,a=e.createShape({type:`bpmn:ParallelGateway`},{x:i.x+P,y:r(i).y},i.parent),o=je(i,[]);t.makeSpace(o,[],{x:P,y:0},`e`,0);let s=r(a);i.incoming.map(e=>e).forEach(t=>e.reconnectEnd(t,a,s)),e.connect(a,i)}}ke.$inject=[`modeling`,`spaceTool`];function Ae(e,t,n,i){let a=[],o=n.createShape({type:`bpmn:ParallelGateway`});o.x=e.x+P-o.width/2,o.y=r(e).y-o.height/2,a.push(o);let s=n.createConnection({type:`bpmn:SequenceFlow`});s.waypoints=i.layoutConnection(s,{source:o,target:e}),a.push(s),e.incoming.forEach(e=>{let t=n.createConnection({type:`bpmn:SequenceFlow`}),s=n.createShape({type:e.source.type});s.x=e.source.x+P,s.y=e.source.y;let c=r(o);t.waypoints=i.layoutConnection(t,{source:s,target:o,connectionEnd:c}),a.push(t)});let c={x:P,y:0},l=je(e,[]);l.forEach(e=>e.incoming.forEach(e=>l.push(e)));let u=l.map(e=>({element:e,delta:c}));t.create({created:a,moved:u,removed:e.incoming})}function je(e,t){return e.incoming.forEach(n=>{let r=n.source;r.x<e.x&&!t.includes(r)&&(t.push(r),r.label&&t.push(r.label),je(r,t))}),t}function Me(e){this.preExecute=function(t){let n=t.problematicEndEvent,i=r(n);n.incoming.slice(1).forEach(t=>{let a=e.createShape({type:`bpmn:EndEvent`},{x:i.x,y:r(t.source).y},n.parent);e.reconnectEnd(t,a,{x:a.x,y:a.y}),e.layoutConnection(t,{waypoints:[]})});let a=n.incoming[0].source,o={x:0,y:r(a).y-Math.round(n.height/2)-n.y};e.moveShape(n,o)}}Me.$inject=[`modeling`];function Ne(e,t,n,i){let a=[];e.incoming.forEach(t=>{let o=n.createShape({type:`bpmn:EndEvent`});o.x=e.x,o.y=r(t.source).y-Math.round(e.height/2),a.push(o);let s=n.createConnection({type:`bpmn:SequenceFlow`});s.waypoints=i.layoutConnection(s,{source:t.source,target:o}),a.push(s)}),t.create({created:a,removed:e.incoming})}var Pe=`quick-fix-note`,Fe=`PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIGhlaWdodD0iMjQiIHZpZXdCb3g9IjAgLTk2MCA5NjAgOTYwIiB3aWR0aD0iMjQiIGZpbGw9IndoaXRlIj48cGF0aCBkPSJNNDgwLTgwcS0zMyAwLTU2LjUtMjMuNVQ0MDAtMTYwaDE2MHEwIDMzLTIzLjUgNTYuNVQ0ODAtODBaTTMyMC0yMDB2LTgwaDMyMHY4MEgzMjBabTEwLTEyMHEtNjktNDEtMTA5LjUtMTEwVDE4MC01ODBxMC0xMjUgODcuNS0yMTIuNVQ0ODAtODgwcTEyNSAwIDIxMi41IDg3LjVUNzgwLTU4MHEwIDgxLTQwLjUgMTUwVDYzMC0zMjBIMzMwWm0yNC04MGgyNTJxNDUtMzIgNjkuNS03OVQ3MDAtNTgwcTAtOTItNjQtMTU2dC0xNTYtNjRxLTkyIDAtMTU2IDY0dC02NCAxNTZxMCA1NCAyNC41IDEwMXQ2OS41IDc5Wm0xMjYgMFoiLz48L3N2Zz4=`;function Ie(e,t,n,r,a,o,s,c,l,u){n.on(D,e=>{let t=document.getElementsByClassName(`djs-overlay-container`).item(0);e.active?t.classList.add(`quick-fixes-hide`):t.classList.remove(`quick-fixes-hide`)}),o.registerHandler(`addSubsequentExclusiveGatewayCommand`,Ee),o.registerHandler(`addPrecedingParallelGatewayCommand`,ke),o.registerHandler(`addEndEventsForEachIncFlowCommand`,Me),n.on(`analysis.done`,e=>{r.remove({type:Pe}),e.property_results.filter(e=>!e.fulfilled).forEach(t=>{t.property===`Safeness`&&ce(t.problematic_elements[0],t),t.property===`ProperCompletion`&&oe(t.problematic_elements[0]),t.property===`OptionToComplete`&&ie(t,e.property_results),t.property===`NoDeadActivities`&&ne(t)})});function d(e,t){let n;return e.parent.children.filter(n=>n.x<e.x&&g(n,`bpmn:FlowNode`)&&n.type!==`label`&&f(n)&&!h(n,[`bpmn:EndEvent`])&&!t.includes(n.id)).forEach(t=>{if(!n)n=t;else{let r=e.x-n.x;e.x-t.x<r&&(n=t)}}),n}function f(e){return g(e,`bpmn:StartEvent`)||e.incoming.length>0}function p(e,t){let n=d(e,t);n&&k(e,{top:-45,left:T(e)},`Click to add an incoming sequence flow to fix the dead Activity.`,()=>{a.connect(n,e)},()=>{let t=l.createConnection({type:`bpmn:SequenceFlow`});t.waypoints=u.layoutConnection(t,{source:n,target:e}),s.create({created:[t]})})}function m(e,t,n){let r=_(e,t);if(r){let t={getConnection:()=>l.createConnection({type:`bpmn:MessageFlow`})};k(e,{top:-45,left:T(e)},`Click to add incoming message flow to fix ${n}`,()=>{a.connect(r,e)},()=>{t.connectionPreviewGfx=void 0,c.drawPreview(t,!0,{source:r,target:e})},()=>{c.cleanUp(t)})}}function _(e,t){if(!e.parent.parent)return;let n=e.parent.parent.children.filter(t=>g(t,`bpmn:Participant`)&&t!==e.parent).flatMap(e=>e.children).filter(e=>e.type!==`label`&&h(e,[`bpmn:IntermediateThrowEvent`,`bpmn:EndEvent`])&&ee(e,`bpmn:MessageEventDefinition`)||g(e,`bpmn:SendTask`)&&!t.includes(e.id)),r,a;return n.forEach(t=>{if(!r)r=t,a=i(e,r);else{let n=i(e,t);n<a&&(r=t,a=n)}}),r}function te(e){return e.incoming.filter(e=>g(e,`bpmn:SequenceFlow`)).length}function v(e){return e.incoming.filter(e=>g(e,`bpmn:MessageFlow`)).length}function ne(e){e.problematic_elements.forEach(n=>{let r=t.get(n);if(te(r)===0){p(r,e.problematic_elements);return}g(r,`bpmn:ReceiveTask`)&&v(r)===0&&m(r,e.problematic_elements,`the dead Receive Task.`)})}function re(e){k(e,{top:-45,left:7.5},`Click to change gateway to parallel to guarantee termination.`,()=>{j(e)},()=>{})}function ie(e,t){if(!e.counter_example)return;let n=e.counter_example.transitions.slice(-1).pop();n&&n.next_state.snapshots.forEach(e=>{x(e.tokens),y(e.tokens,t)})}function y(e,n){let r=n.find(e=>e.property===xe);e.forEach((e,n)=>{let i=t.get(n);if(!i)return;let a=i.target;g(a,`bpmn:IntermediateCatchEvent`)&&ee(a,`bpmn:MessageEventDefinition`)&&v(a)===0&&m(a,r.problematic_elements,`the blocking Message Catch Event.`)})}function b(e){let n=[];return e.forEach((e,r)=>{let i=t.get(r);if(!i)return;let a=i.target;g(a,`bpmn:ParallelGateway`)&&a.incoming.length>1&&n.push(a)}),n}function x(e){let t=b(e);if(t.length===1){let e=t.pop();ae(e);let n=fe(e);n&&re(n)}}function ae(e){k(e,{top:-45,left:7.5},`Click to change gateway to exclusive to guarantee termination.`,()=>{A(e)},()=>{})}function oe(e){let n=t.get(e);n.incoming.length<=1||k(n,{top:-45,left:0},`Click to create additional end events.`,()=>{o.execute(`addEndEventsForEachIncFlowCommand`,{problematicEndEvent:n})},()=>{Ne(n,s,l,u)})}function S(e,t){let n=e.source;if(n.incoming.length>1&&se(n,t))return n;for(let e of n.incoming){let n=S(e,t);if(n)return n}}function se(e,t){return!e.incoming.some(e=>t.includes(e.id))}function C(e,t,n,r){let i=e.source;if(i.outgoing.length>1&&r(i)&&t.push(i),i.incoming)for(let e of i.incoming)n.includes(e)||(n.push(e),C(e,t,n,r));return t}function ce(e,n){let r=S(t.get(e),n.problematic_elements);if(r){w(r);let e=pe(r);e&&ue(e)}}function w(e){g(e,`bpmn:ExclusiveGateway`)?O(e):E(e)}function T(e){return e.width/2-17}function E(e){k(e,{top:-45,left:T(e)},`Click to add preceding parallel gateway to fix synchronization.`,()=>o.execute(`addPrecedingParallelGatewayCommand`,{unsafeMerge:e}),()=>{Ae(e,s,l,u)})}function le(e){k(e,{top:-45,left:T(e)},`Click to add subsequent exclusive gateway to fix synchronization.`,()=>o.execute(`addSubsequentExclusiveGatewayCommand`,{unsafeCause:e}),()=>{Oe(e,s,l,u)})}function ue(e){g(e,`bpmn:ParallelGateway`)?de(e):le(e)}function de(e){k(e,{top:-45,left:7.5},`Click to change gateway to exclusive to fix synchronization.`,()=>{A(e)},()=>{})}function fe(e){return me(e.incoming.map(e=>C(e,[],[],e=>e.type===`bpmn:ExclusiveGateway`)))}function pe(e){return me(e.incoming.map(e=>C(e,[],[],e=>e.type!==`bpmn:ExclusiveGateway`)))}function me(e){for(let t of e[0])if(e.every(e=>e.includes(t)))return t}function O(e){k(e,{top:-45,left:7.5},`Click to change gateway to parallel to fix synchronization.`,()=>{j(e)},()=>{})}function k(e,t,n,i,a,o=()=>s.cleanUp()){if(he(e))return;r.add(e,Pe,{position:t,html:`<div id=${e.id} class="small-note quick-fix-note tooltip">
               <img alt="quick-fix" src="data:image/svg+xml;base64,${Fe}"/>
               <span class="tooltipText">${n}</span>
           </div>`});let c=document.getElementById(e.id);c&&(c.addEventListener(`click`,()=>{o(),i()}),c.addEventListener(`mouseenter`,()=>{a()}),c.addEventListener(`mouseleave`,()=>{o()}))}function he(e){return r.get({element:e,type:Pe}).length>0}function A(t){e.replaceElement(t,{type:`bpmn:ExclusiveGateway`})}function j(t){e.replaceElement(t,{type:`bpmn:ParallelGateway`})}}Ie.$inject=[`bpmnReplace`,`elementRegistry`,`eventBus`,`overlays`,`modeling`,`commandStack`,`complexPreview`,`connectionPreview`,`elementFactory`,`layouter`];var Le={__init__:[`quickFixes`],quickFixes:[`type`,Ie]};function Re(e,t,n,r,i,a,o){this._notifications=a,t.on(ge,e=>{e.propertyResult&&u(e.propertyResult)});let s={},c;t.on(M,()=>{c&&u(c)}),t.on(`analysis.done`,e=>{e.property_results.forEach(e=>{l(e)})});function l(e){let t=document.getElementById(e.property);if(t&&(t.removeEventListener(`click`,s[e.property]),t.classList.remove(`clickable`),!e.fulfilled&&e.counter_example)){t.classList.add(`clickable`);let n=u.bind(this,e);t.addEventListener(`click`,n),s[e.property]=n}}function u(n){c=n,e.clearAnimations(),r.clearTokenCounts(),i.clearMessageCounts(),t.fire(D,{active:!0}),t.fire(_e,{propertyResult:n}),a.showNotification({text:`Visualizing execution example started.`}),d(n)}function d(e){p(e.property,{snapshots:[],messages:new Map},e.counter_example.start_state,e.counter_example.transitions,-1)}function f(e,r,i,o){if(o>=i.length){a.showNotification({text:`Visualizing execution example finished.`});return}let s=i[o],c=n.get(s.label);if(!c){f(e,r,i,o+1);return}t.fire(j,{element:c,property:e}),p(e,r,s.next_state,i,o)}function p(t,a,s,c,l){let u=h(s.snapshots,a.snapshots),d=g(s,a);if(m(s,a).forEach((e,t)=>{let r=n.get(t);if(r&&r.target){let t={primary:`#999`,auxiliary:`#FFF`};for(let n=0;n<e;n++)i.decreaseMessageCount(r.target,t)}}),d.size===0&&u.every(e=>e.tokens.size===0)){f(t,s,c,l+1);return}let p=0;d.forEach((r,a)=>{let o=n.get(a);if(!o)return;let u={element:o,colors:{primary:`#999`,auxiliary:`#FFF`}};for(let n=0;n<r;n++)p++,e.animate(o,u,()=>{p--,o.target&&i.increaseMessageCount(o.target,u.colors),p===0&&f(t,s,c,l+1)})}),u.forEach(i=>{i.tokens.forEach((a,u)=>{let d=n.get(u);if(!d)return;let m={element:d,colors:o.getColor(i.id)};for(let n=0;n<a;n++)p++,r.decreaseTokenCount(d.source,m.colors),e.animate(d,m,()=>{p--,r.increaseTokenCount(d.target,m.colors),p===0&&f(t,s,c,l+1)})})})}function m(e,t){let n=new Map;return t.messages.forEach((t,r)=>{let i=t-(e.messages.get(r)||0);i>0&&n.set(r,i)}),n}function h(e,t){let n=e.map(e=>({tokens:new Map(e.tokens),id:e.id}));return t.forEach(e=>{e.tokens.forEach((t,r)=>{let i=n.find(t=>t.id===e.id),a=i.tokens.get(r)-t;a>0?i.tokens.set(r,a):i.tokens.delete(r)})}),n}function g(e,t){let n=new Map(e.messages);return t.messages.forEach((e,t)=>{let r=n.get(t)-e;r>0?n.set(t,r):n.delete(t)}),n}}Re.$inject=[`animation`,`eventBus`,`elementRegistry`,`tokenCount`,`messageCount`,`notifications`,`tokenColors`];var ze=getComputedStyle(document.documentElement),Be=ze.getPropertyValue(`--token-simulation-green-base-44`),Ve=ze.getPropertyValue(`--token-simulation-white`);function He(){}function Ue(e,t){return t.length===2?qe:e===1?Ge:e===t.length-1?Ke:We}var We=function(e){return e},Ge=function(e){return-Math.cos(e*Math.PI/2)+1},Ke=function(e){return Math.sin(e*Math.PI/2)},qe=function(e){return-Math.cos(e*Math.PI)/2+.5},F=20;function I(e,t){this._eventBus=t,this._canvas=e,this._randomize=!0,this._animations=new Set,this._speed=1,t.on(M,()=>{this.clearAnimations()}),t.on(k,()=>{this.pause()}),t.on(O,()=>{this.play()}),t.on(D,e=>{e.active||this.clearAnimations()})}I.prototype.animate=function(e,t,n){this.createAnimation(e,t,n)},I.prototype.pause=function(){this.each(e=>e.pause())},I.prototype.play=function(){this.each(e=>e.play())},I.prototype.each=function(e){this._animations.forEach(e)},I.prototype.createAnimation=function(e,t,n=He){let r=this._getGroup(t);if(!r)return;let i=new L(this._createTokenGfx(r,t),e.waypoints,()=>{this._animations.delete(i),n()});return i.setSpeed(this.getAnimationSpeed()),i.scope=t,i.element=e,this._animations.add(i),this._eventBus.fire(he,{animation:i}),i.play(),i},I.prototype.setAnimationSpeed=function(e){this._speed=e,this.each(t=>t.setSpeed(e)),this._eventBus.fire(A,{speed:e})},I.prototype.getAnimationSpeed=function(){return this._speed},I.prototype.clearAnimations=function(e){this.each(t=>{(!e||t.scope===e)&&t.remove()})},I.prototype._createTokenGfx=function(e,t){return l(p(this._getTokenSVG(t).trim()),e)},I.prototype._getTokenSVG=function(e){let t=e.colors||{primary:Be,auxiliary:Ve};return`
    <g class="bts-token">
      <circle
        class="bts-circle"
        r="${F/2}"
        cx="${F/2}"
        cy="${F/2}"
        fill="${t.primary}"
      />
      <text
        class="bts-text"
        transform="translate(10, 14)"
        text-anchor="middle"
        fill="${t.auxiliary}"
      >1</text>
    </g>
  `},I.prototype._getGroup=function(e){let t=this._canvas,n=t.findRoot(e.element),r=t._findPlaneForRoot(n).layer,i=c(`.bts-animation-tokens`,r);return i||(i=p(`<g class="bts-animation-tokens" />`),l(i,r)),i},I.$inject=[`canvas`,`eventBus`];function L(e,t,n){this.gfx=e,this.waypoints=t,this.done=n,this._paused=!0,this._t=0,this._parts=[],this.create()}L.prototype.pause=function(){this._paused=!0},L.prototype.play=function(){this._paused&&(this._paused=!1,this.tick(0)),this.schedule()},L.prototype.schedule=function(){if(this._paused||this._scheduled)return;let e=Date.now();this._scheduled=!0,requestAnimationFrame(()=>{this._scheduled=!1,!this._paused&&(this.tick((Date.now()-e)*this._speed),this.schedule())})},L.prototype.tick=function(e){let t=this._t+=e,n=this._parts.find(e=>e.startTime<=t&&e.endTime>t);if(!n)return this.remove();let r=t-n.startTime,i=n.length*n.easing(r/n.duration),a=n.startLength+i,o=this._path.getPointAtLength(a);this.move(o.x,o.y)},L.prototype.move=function(e,t){o(this.gfx,`transform`,`translate(${e}, ${t})`)},L.prototype.create=function(){let e=this.waypoints,t=e.reduce((t,n,r)=>{let i=e[r-1];if(i){let a=t[t.length-1],o=a&&a.endLength||0,s=Xe(i,n);t.push({startLength:o,endLength:o+s,length:s,easing:Ue(r,e)})}return t},[]),n=t.reduce(function(e,t){return e+t.length},0),r=e.reduce((e,t,n)=>{let r=t.x-F/2,i=t.y-F/2;return e.push([n>0?`L`:`M`,r,i]),e},[]).flat().join(` `),i=Je(n,this._randomize);this._parts=t.reduce((e,t,r)=>{let a=i/n*t.length,o=r>0?e[r-1].endTime:0,s=o+a;return[...e,{...t,startTime:o,endTime:s,duration:a}]},[]),this._path=p(`<path d="${r}" />`),this._t=0},L.prototype.remove=function(){this.pause(),u(this.gfx),this.done()},L.prototype.setSpeed=function(e){this._speed=e};function Je(e,t=!1){return Math.log(e)*(t?Ye(250,300):250)}function Ye(e,t){return e+Math.floor(Math.random()*(t-e))}function Xe(e,t){return Math.sqrt((e.x-t.x)**2+(e.y-t.y)**2)}var Ze={animation:[`type`,I]};function Qe(e,n){return t(_(e).eventDefinitions,e=>g(e,n))}var $e=500,et=10,tt=-15,nt=`--token-simulation-green-base-44`,rt=`--token-simulation-white`,it=`token-count`;function R(e,t,n){this._overlays=t,this.overlayIdsAndCount={},e.on(M,()=>{this.clearTokenCounts()}),e.on(D,$e,e=>{e.active||this.clearTokenCounts()}),e.on(j,e=>{let t=e.element;h(t,[`bpmn:EndEvent`,`bpmn:EventBasedGateway`])&&this.decreaseTokenCount(t,n.getColorForElement(t)),g(t,`bpmn:ParallelGateway`)&&this.decreaseTokenCountBy(t,t.incoming.length-1,n.getColorForElement(t)),Qe(_(t),`bpmn:TerminateEventDefinition`)&&t.parent.children.forEach(e=>this.removeAllTokens(e))})}R.prototype.addTokenCountOverlay=function(e,t,n){let r=a(`
    <div class="bts-token-count-parent">
      ${this._getTokenHTML(e,t,n)}
    </div>
  `),i={bottom:et,left:tt};return this._overlays.add(e,it,{position:i,html:r,show:{minZoom:.5}})},R.prototype.increaseTokenCount=function(e,t){let n=1,r=this.overlayIdsAndCount[e.id];r&&(this._overlays.remove(r.id),n=r.count+1);let i=this.addTokenCountOverlay(e,n,t);this.overlayIdsAndCount[e.id]={id:i,count:n}},R.prototype.clearTokenCounts=function(){this._overlays.remove({type:it}),this.overlayIdsAndCount={}},R.prototype.removeAllTokens=function(e){let t=this.overlayIdsAndCount[e.id];t&&(this._overlays.remove(t.id),delete this.overlayIdsAndCount[e.id])},R.prototype.decreaseTokenCountBy=function(e,t,n){let r=this.overlayIdsAndCount[e.id];if(r)if(this._overlays.remove(r.id),r.count>t){let i=r.count-t,a=this.addTokenCountOverlay(e,i,n);this.overlayIdsAndCount[e.id]={id:a,count:i}}else delete this.overlayIdsAndCount[e.id]},R.prototype.decreaseTokenCount=function(e,t){this.decreaseTokenCountBy(e,1,t)},R.prototype._getTokenHTML=function(e,t,n){return n||=this._getDefaultColors(),`
    <div class="bts-token-count waiting"
         style="color: ${n.auxiliary}; background: ${n.primary}">
      ${t}
    </div>
  `},R.prototype._getDefaultColors=function(){return{primary:nt,auxiliary:rt}},R.$inject=[`eventBus`,`overlays`,`tokenColors`];var at={__init__:[`tokenCount`],tokenCount:[`type`,R]},ot=500,st=-10,ct=-5,lt=`message-count`,ut=`<svg class="bts-count-icon" viewBox="0 0 16 16" fill="currentColor" aria-hidden="true">
  <path d="M1 4v9h14V4H1zm1.5 1h11L8 8.5 2.5 5zm-.5 7V5.5l6 4 6-4V12H2z"/>
</svg>`,dt=`--token-simulation-grey-lighten-56`,ft=`--token-simulation-white`;function z(e,t){this._overlays=t,this.overlayIdsAndCount={},e.on(M,()=>{this.clearMessageCounts()}),e.on(D,ot,e=>{e.active||this.clearMessageCounts()})}z.prototype.addMessageCountOverlay=function(e,t,n){let r=a(`
    <div class="bts-message-count-parent">
      ${this._getMessageHTML(t,n)}
    </div>
  `),i={top:st,right:ct};return this._overlays.add(e,lt,{position:i,html:r,show:{minZoom:.5}})},z.prototype.increaseMessageCount=function(e,t){let n=1,r=this.overlayIdsAndCount[e.id];r&&(this._overlays.remove(r.id),n=r.count+1);let i=this.addMessageCountOverlay(e,n,t);this.overlayIdsAndCount[e.id]={id:i,count:n}},z.prototype.clearMessageCounts=function(){this._overlays.remove({type:lt}),this.overlayIdsAndCount={}},z.prototype.decreaseMessageCountBy=function(e,t,n){let r=this.overlayIdsAndCount[e.id];if(r)if(this._overlays.remove(r.id),r.count>t){let i=r.count-t,a=this.addMessageCountOverlay(e,i,n);this.overlayIdsAndCount[e.id]={id:a,count:i}}else delete this.overlayIdsAndCount[e.id]},z.prototype.decreaseMessageCount=function(e,t){this.decreaseMessageCountBy(e,1,t)},z.prototype._getMessageHTML=function(e,t){return t||=this._getDefaultColors(),`
    <div class="bts-message-count waiting"
         style="color: ${t.auxiliary}; background: ${t.primary}">
      ${e}
      ${ut}
    </div>
  `},z.prototype._getDefaultColors=function(){return{primary:`var(${dt})`,auxiliary:`var(${ft})`}},z.$inject=[`eventBus`,`overlays`];var pt={__init__:[`messageCount`],messageCount:[`type`,z]},mt=(0,e(v(),1).default)({count:60}).filter(e=>gt(e)<200),ht=0,B=new Map;function gt(e){let t=parseInt(e.substring(1,3),16),n=parseInt(e.substring(3,5),16),r=parseInt(e.substring(5,7),16);return(t*299+n*587+r*114)/1e3}function V(e){e.on(ge,()=>{ht=0,B.clear()})}V.$inject=[`eventBus`],V.prototype.getColor=function(e){let t=B.get(e);if(t)return t;let n=mt[ht++%mt.length];return t={primary:n,auxiliary:gt(n)>=128?`#111`:`#fff`},B.set(e,t),t},V.prototype.getColorForElement=function(e){return e.parent.businessObject.processRef?this.getColor(e.parent.businessObject.processRef.id):this.getColor(e.parent.id)};var _t={__init__:[`tokenColors`],tokenColors:[`type`,V]},vt=`<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><!-- Font Awesome Free 5.15.4 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) --><path fill="currentColor" d="M12.83 352h262.34A12.82 12.82 0 0 0 288 339.17v-38.34A12.82 12.82 0 0 0 275.17 288H12.83A12.82 12.82 0 0 0 0 300.83v38.34A12.82 12.82 0 0 0 12.83 352zm0-256h262.34A12.82 12.82 0 0 0 288 83.17V44.83A12.82 12.82 0 0 0 275.17 32H12.83A12.82 12.82 0 0 0 0 44.83v38.34A12.82 12.82 0 0 0 12.83 96zM432 160H16a16 16 0 0 0-16 16v32a16 16 0 0 0 16 16h416a16 16 0 0 0 16-16v-32a16 16 0 0 0-16-16zm0 256H16a16 16 0 0 0-16 16v32a16 16 0 0 0 16 16h416a16 16 0 0 0 16-16v-32a16 16 0 0 0-16-16z"/></svg>`,yt=`<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 192 512"><!-- Font Awesome Free 5.15.4 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) --><path fill="currentColor" d="M20 424.229h20V279.771H20c-11.046 0-20-8.954-20-20V212c0-11.046 8.954-20 20-20h112c11.046 0 20 8.954 20 20v212.229h20c11.046 0 20 8.954 20 20V492c0 11.046-8.954 20-20 20H20c-11.046 0-20-8.954-20-20v-47.771c0-11.046 8.954-20 20-20zM96 0C56.235 0 24 32.235 24 72s32.235 72 72 72 72-32.235 72-72S135.764 0 96 0z"/></svg>`,bt=`<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 352 512"><!-- Font Awesome Free 5.15.4 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) --><path fill="currentColor" d="M242.72 256l100.07-100.07c12.28-12.28 12.28-32.19 0-44.48l-22.24-22.24c-12.28-12.28-32.19-12.28-44.48 0L176 189.28 75.93 89.21c-12.28-12.28-32.19-12.28-44.48 0L9.21 111.45c-12.28 12.28-12.28 32.19 0 44.48L109.28 256 9.21 356.07c-12.28 12.28-12.28 32.19 0 44.48l22.24 22.24c12.28 12.28 32.2 12.28 44.48 0L176 322.72l100.07 100.07c12.28 12.28 32.2 12.28 44.48 0l22.24-22.24c12.28-12.28 12.28-32.19 0-44.48L242.72 256z"/></svg>`,xt=`<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><!-- Font Awesome Free 5.15.4 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) --><path fill="currentColor" d="M440.65 12.57l4 82.77A247.16 247.16 0 0 0 255.83 8C134.73 8 33.91 94.92 12.29 209.82A12 12 0 0 0 24.09 224h49.05a12 12 0 0 0 11.67-9.26 175.91 175.91 0 0 1 317-56.94l-101.46-4.86a12 12 0 0 0-12.57 12v47.41a12 12 0 0 0 12 12H500a12 12 0 0 0 12-12V12a12 12 0 0 0-12-12h-47.37a12 12 0 0 0-11.98 12.57zM255.83 432a175.61 175.61 0 0 1-146-77.8l101.8 4.87a12 12 0 0 0 12.57-12v-47.4a12 12 0 0 0-12-12H12a12 12 0 0 0-12 12V500a12 12 0 0 0 12 12h47.35a12 12 0 0 0 12-12.6l-4.15-82.57A247.17 247.17 0 0 0 255.83 504c121.11 0 221.93-86.92 243.55-201.82a12 12 0 0 0-11.8-14.18h-49.05a12 12 0 0 0-11.67 9.26A175.86 175.86 0 0 1 255.83 432z"/></svg>`,St=`<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><!-- Font Awesome Free 5.15.4 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) --><path fill="currentColor" d="M144 479H48c-26.5 0-48-21.5-48-48V79c0-26.5 21.5-48 48-48h96c26.5 0 48 21.5 48 48v352c0 26.5-21.5 48-48 48zm304-48V79c0-26.5-21.5-48-48-48h-96c-26.5 0-48 21.5-48 48v352c0 26.5 21.5 48 48 48h96c26.5 0 48-21.5 48-48z"/></svg>`,Ct=`<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><!-- Adapted from Font Awesome Free 5.15.4 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) --><path fill="currentColor" d="M424.4 214.7L72.4 6.6C43.8-10.3 0 6.1 0 47.9V464c0 37.5 40.7 60.1 72.4 41.3l352-208c31.4-18.5 31.5-64.1 0-82.6z"/></svg>`,wt=`<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><!-- Font Awesome Free 5.15.4 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) --><path fill="currentColor" d="M504 256c0 136.967-111.033 248-248 248S8 392.967 8 256 119.033 8 256 8s248 111.033 248 248zM227.314 387.314l184-184c6.248-6.248 6.248-16.379 0-22.627l-22.627-22.627c-6.248-6.249-16.379-6.249-22.628 0L216 308.118l-70.059-70.059c-6.248-6.248-16.379-6.248-22.628 0l-22.627 22.627c-6.248 6.248-6.248 16.379 0 22.627l104 104c6.249 6.249 16.379 6.249 22.628.001z"/></svg>`,Tt=`<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 576 512"><!-- Font Awesome Free 5.15.4 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) --><path fill="currentColor" d="M288 32C128.94 32 0 160.94 0 320c0 52.8 14.25 102.26 39.06 144.8 5.61 9.62 16.3 15.2 27.44 15.2h443c11.14 0 21.83-5.58 27.44-15.2C561.75 422.26 576 372.8 576 320c0-159.06-128.94-288-288-288zm0 64c14.71 0 26.58 10.13 30.32 23.65-1.11 2.26-2.64 4.23-3.45 6.67l-9.22 27.67c-5.13 3.49-10.97 6.01-17.64 6.01-17.67 0-32-14.33-32-32S270.33 96 288 96zM96 384c-17.67 0-32-14.33-32-32s14.33-32 32-32 32 14.33 32 32-14.33 32-32 32zm48-160c-17.67 0-32-14.33-32-32s14.33-32 32-32 32 14.33 32 32-14.33 32-32 32zm246.77-72.41l-61.33 184C343.13 347.33 352 364.54 352 384c0 11.72-3.38 22.55-8.88 32H232.88c-5.5-9.45-8.88-20.28-8.88-32 0-33.94 26.5-61.43 59.9-63.59l61.34-184.01c4.17-12.56 17.73-19.45 30.36-15.17 12.57 4.19 19.35 17.79 15.17 30.36zm14.66 57.2l15.52-46.55c3.47-1.29 7.13-2.23 11.05-2.23 17.67 0 32 14.33 32 32s-14.33 32-32 32c-11.38-.01-20.89-6.28-26.57-15.22zM480 384c-17.67 0-32-14.33-32-32s14.33-32 32-32 32 14.33 32 32-14.33 32-32 32z"/></svg>`,Et=`<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 512"><!-- Font Awesome Free 5.15.4 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) --><path fill="currentColor" d="M224.3 273l-136 136c-9.4 9.4-24.6 9.4-33.9 0l-22.6-22.6c-9.4-9.4-9.4-24.6 0-33.9l96.4-96.4-96.4-96.4c-9.4-9.4-9.4-24.6 0-33.9L54.3 103c9.4-9.4 24.6-9.4 33.9 0l136 136c9.5 9.4 9.5 24.6.1 34z"/></svg>`;function H(e){return function(t=``){return`<span class="bts-icon ${t}">${e}</span>`}}var Dt=H(vt),Ot=H(yt),kt=H(bt),At=H(xt),jt=H(St),Mt=H(Ct),Nt=H(Tt),Pt=H(Et),Ft=H(wt),It=2e3,Lt=Ot();function U(e,t){this._eventBus=e,this._canvas=t,this._init(),e.on([D,M],()=>{this.clear()})}U.prototype._init=function(){this.container=a(`<div class="bts-notifications"></div>`),this._canvas.getContainer().appendChild(this.container)},U.prototype.showNotification=function(e){let{text:t,type:n=`info`,icon:r=Lt,scope:i,ttl:o=It}=e,s=r.startsWith(`<`)?r:`<i class="${r}"></i>`,c=i&&i.colors,l=c?`style="color: ${c.auxiliary}; background: ${c.primary}"`:``,u=a(`
    <div class="bts-notification ${n}">
      <span class="bts-icon">${s}</span>
      <span class="bts-text" title="${t}">${t}</span>
      ${i?`<span class="bts-scope" ${l}>${i.id}</span>`:``}
    </div>
  `);for(this.container.appendChild(u);this.container.children.length>5;)this.container.children[0].remove();setTimeout(function(){u.remove()},o)},U.prototype.clear=function(){for(;this.container.children.length;)this.container.children[0].remove()},U.$inject=[`eventBus`,`canvas`];var Rt={notifications:[`type`,U]},zt=Ot();function Bt(e){let t=e.di.bpmnElement.name;return t?n(t):``}function W(e,t,n,r,i){this._notifications=t,this._canvas=n,this._tokenSimulationPalette=r,this._init();function o(e){return e.parent.businessObject.name?e.parent.businessObject.name.substring(0,20):e.parent.id.substring(0,7)}e.on(j,e=>{let{element:t}=e,n=Bt(t),r={id:o(t),colors:i.getColorForElement(t)};if(g(t,`bpmn:BusinessRuleTask`)){this.log({text:n||`Business Rule Task`,icon:`bpmn-icon-business-rule`,scope:r});return}if(g(t,`bpmn:SendTask`)){this.log({text:n||`Send Task`,icon:`bpmn-icon-send-task`,scope:r});return}if(g(t,`bpmn:SubProcess`)){this.log({text:n||`Sub-process`,icon:`bpmn-icon-subprocess-collapsed`,scope:r});return}if(g(t,`bpmn:ReceiveTask`)){this.log({text:n||`Receive Task`,icon:`bpmn-icon-receive-task`,scope:r});return}if(g(t,`bpmn:IntermediateThrowEvent`)){this.log({text:n||`Intermediate Throw Event`,icon:Ht(t,`throw`),scope:r});return}if(g(t,`bpmn:IntermediateCatchEvent`)){this.log({text:n||`Intermediate Catch Event`,icon:Ht(t,`catch`),scope:r});return}if(g(t,`bpmn:BoundaryEvent`)){this.log({text:n||`Boundary Event`,icon:`bpmn-icon-intermediate-event-none`,scope:r});return}if(g(t,`bpmn:ManualTask`)){this.log({text:n||`Manual Task`,icon:`bpmn-icon-manual`,scope:r});return}if(g(t,`bpmn:ScriptTask`)){this.log({text:n||`Script Task`,icon:`bpmn-icon-script`,scope:r});return}if(g(t,`bpmn:ServiceTask`)){this.log({text:n||`Service Task`,icon:`bpmn-icon-service`,scope:r});return}if(g(t,`bpmn:UserTask`)){this.log({text:n||`User Task`,icon:`bpmn-icon-user`,scope:r});return}if(g(t,`bpmn:Task`)){this.log({text:n||`Task`,icon:`bpmn-icon-task`,scope:r});return}if(g(t,`bpmn:ExclusiveGateway`)){this.log({text:n||`Exclusive Gateway`,icon:`bpmn-icon-gateway-xor`,scope:r});return}if(g(t,`bpmn:ParallelGateway`)){this.log({text:n||`Parallel Gateway`,icon:`bpmn-icon-gateway-parallel`,scope:r});return}if(g(t,`bpmn:EventBasedGateway`)){this.log({text:n||`Event-based Gateway`,icon:`bpmn-icon-gateway-eventbased`,scope:r});return}if(g(t,`bpmn:StartEvent`)){this.log({text:n||`Start Event`,icon:`bpmn-icon-start-event-${Ut(t)}`,scope:r});return}g(t,`bpmn:EndEvent`)&&this.log({text:n||`End Event`,icon:`bpmn-icon-end-event-${Ut(t)}`,scope:r})}),e.on(D,e=>{e.active?this.clear():this.toggle(!1)}),e.on(M,()=>{this.clear()}),e.on(_e,e=>{this.clear(),this.toggle(!0);let t=a(`<p class="bts-entry placeholder">${Vt(e.propertyResult.property)}</p>`);this._content.prepend(t)})}function Vt(e){switch(e){case be:return`Unique end event execution`;case ve:return`Synchronization`;case ye:return`Guaranteed termination`}return``}function Ht(e,t){let n=Ut(e);return n===`none`?`bpmn-icon-intermediate-event-none`:`bpmn-icon-intermediate-event-${t}-${n}`}function Ut(e){let t=_(e);if(t.eventDefinitions.length===0)return`none`;let n=t.eventDefinitions[0];return g(n,`bpmn:MessageEventDefinition`)?`message`:g(n,`bpmn:TimerEventDefinition`)?`timer`:g(n,`bpmn:SignalEventDefinition`)?`signal`:g(n,`bpmn:ErrorEventDefinition`)?`error`:g(n,`bpmn:EscalationEventDefinition`)?`escalation`:g(n,`bpmn:CompensateEventDefinition`)?`compensation`:g(n,`bpmn:ConditionalEventDefinition`)?`condition`:g(n,`bpmn:LinkEventDefinition`)?`link`:g(n,`bpmn:CancelEventDefinition`)?`cancel`:g(n,`bpmn:TerminateEventDefinition`)?`terminate`:`none`}W.prototype._init=function(){this._container=a(`
    <div class="bts-log hidden djs-scrollable">
      <div class="bts-header">
        ${Dt(`bts-log-icon`)}
        Execution Log
        <button class="bts-close">
          ${kt()}
        </button>
      </div>
      <div class="bts-content">
        <p class="bts-entry placeholder">No Entries</p>
      </div>
    </div>
  `),this._placeholder=c(`.bts-placeholder`,this._container),this._content=c(`.bts-content`,this._container),d.bind(this._content,`mousedown`,e=>{e.stopPropagation()}),this._close=c(`.bts-close`,this._container),d.bind(this._close,`click`,()=>{this.toggle(!1)}),this._icon=c(`.bts-log-icon`,this._container),d.bind(this._icon,`click`,()=>{this.toggle()}),this._canvas.getContainer().appendChild(this._container),this.paletteEntry=a(`
    <div class="bts-entry" title="Toggle Execution Log">
      ${Dt()}
    </div>
  `),d.bind(this.paletteEntry,`click`,()=>{this.toggle()}),this._tokenSimulationPalette.addEntry(this.paletteEntry,3)},W.prototype.isShown=function(){let e=this._container;return!s(e).has(`hidden`)},W.prototype.toggle=function(e=!this.isShown()){let t=this._container;e?s(t).remove(`hidden`):s(t).add(`hidden`)},W.prototype.log=function(e){let{text:t,type:n=`info`,icon:r=zt,scope:i}=e,o=this._content;s(this._placeholder).add(`hidden`),this.isShown()||this._notifications.showNotification(e);let c=r.startsWith(`<`)?r:`<i class="${r}"></i>`,l=i&&i.colors,u=l?`style="background: ${l.primary}; color: ${l.auxiliary}"`:``,d=a(`
    <p class="bts-entry ${n}" ${i?`data-scope-id="${i.id}"`:``}>
      <span class="bts-icon">${c}</span>
      <span class="bts-text" title="${t}">${t}</span>
      ${i?`<span class="bts-scope" data-scope-id="${i.id}" ${u}>${i.id}</span>`:``}
    </p>
  `),f=Math.abs(o.clientHeight+o.scrollTop-o.scrollHeight)<2;o.appendChild(d),f&&(o.scrollTop=o.scrollHeight)},W.prototype.clear=function(){for(;this._content.firstChild;)this._content.removeChild(this._content.firstChild);this._placeholder=a(`<p class="bts-entry placeholder">No Entries</p>`),this._content.appendChild(this._placeholder)},W.$inject=[`eventBus`,`notifications`,`canvas`,`tokenSimulationPalette`,`tokenColors`];var Wt={__depends__:[Rt],__init__:[`log`],log:[`type`,W]},Gt=10001;function Kt(e,t,n,r,i,a,o){let s=!1;e.on(D,Gt,e=>{s=e.active,s&&(r.cancel(),t.close(),n.cancel()),o._update()});function c(e,t,n){let r=e[t];e[t]=function(){return n.call(this,r,arguments)}}function l(e,t){c(e,t,function(e,t){if(!s)return e.apply(this,t)})}function u(e,t){c(e,t,function(e,t){if(s)throw Error(`model is read-only`);return e.apply(this,t)})}l(t,`open`),l(n,`init`),l(r,`activate`),l(n,`init`),l(r,`activate`),u(a,`moveShape`),u(a,`updateAttachment`),u(a,`moveElements`),u(a,`moveConnection`),u(a,`layoutConnection`),u(a,`createConnection`),u(a,`createShape`),u(a,`createLabel`),u(a,`appendShape`),u(a,`removeElements`),u(a,`distributeElements`),u(a,`removeShape`),u(a,`removeConnection`),u(a,`replaceShape`),u(a,`pasteElements`),u(a,`alignElements`),u(a,`resizeShape`),u(a,`createSpace`),u(a,`updateWaypoints`),u(a,`reconnectStart`),u(a,`reconnectEnd`),c(i,`trigger`,function(e,t){let n=t[0];if(!(s&&qt([`undo`,`redo`,`copy`,`paste`,`removeSelection`,`spaceTool`,`lassoTool`,`globalConnectTool`,`distributeElements`,`alignElements`,`directEditing`],n)))return e.apply(this,t)})}Kt.$inject=[`eventBus`,`contextPad`,`dragging`,`directEditing`,`editorActions`,`modeling`,`palette`];function qt(e,t){return e.indexOf(t)>-1}var Jt={__init__:[`disableModeling`],disableModeling:[`type`,Kt]};function G(e,t){let n=this;this._canvas=t,this.entries=[],this._init(),e.on(`diagram.init`,()=>{this._canvasParent=this._canvas.getContainer().parentNode,this._palette=c(`.djs-palette`,this._canvas.getContainer())}),e.on(D,function(e){e.active?(s(n.container).remove(`hidden`),s(n._canvasParent).add(`simulation`),s(n._palette).add(`hidden`)):(s(n.container).add(`hidden`),s(n._canvasParent).remove(`simulation`),s(n._palette).remove(`hidden`))})}G.prototype._init=function(){this.container=a(`<div class="bts-palette hidden"></div>`),this._canvas.getContainer().appendChild(this.container)},G.prototype.addEntry=function(e,t){let n=0;this.entries.forEach(function(e){t>=e.index&&n++}),this.container.insertBefore(e,this.container.childNodes[n]),this.entries.push({entry:e,index:t})},G.$inject=[`eventBus`,`canvas`];var Yt={__init__:[`tokenSimulationPalette`],tokenSimulationPalette:[`type`,G]};function K(e,t){this._eventBus=e,this._tokenSimulationPalette=t,this._init()}K.prototype._init=function(){this._paletteEntry=a(`
    <div class="bts-entry" title="Restart execution example">
      ${At()}
    </div>
  `),d.bind(this._paletteEntry,`click`,()=>{this.restartCounterExample()}),this._tokenSimulationPalette.addEntry(this._paletteEntry,2)},K.prototype.restartCounterExample=function(){this._eventBus.fire(M)},K.$inject=[`eventBus`,`tokenSimulationPalette`];var Xt={__init__:[`restartCounterExample`],restartCounterExample:[`type`,K]},Zt=Mt(),Qt=jt();function q(e,t,n,r){this._eventBus=e,this._tokenSimulationPalette=t,this._notifications=n,this.canvasParent=r.getContainer().parentNode,this.isPaused=!1,this._init()}q.prototype._init=function(){this.paletteEntry=a(`
    <div class="bts-entry" title="Play/Pause Execution">
      ${Qt}
    </div>
  `),d.bind(this.paletteEntry,`click`,this.toggle.bind(this)),this._tokenSimulationPalette.addEntry(this.paletteEntry,1),this._eventBus.on(M,()=>{s(this.paletteEntry).remove(`active`),s(this.canvasParent).remove(`paused`),this.paletteEntry.innerHTML=Qt,this.isPaused=!1})},q.prototype.toggle=function(){this.isPaused?this.unpause():this.pause()},q.prototype.pause=function(){s(this.paletteEntry).add(`active`),s(this.canvasParent).add(`paused`),this.paletteEntry.innerHTML=Zt,this._eventBus.fire(k),this._notifications.showNotification({text:`Pause Execution`}),this.isPaused=!0},q.prototype.unpause=function(){this.isPaused&&=(s(this.paletteEntry).remove(`active`),s(this.canvasParent).remove(`paused`),this.paletteEntry.innerHTML=Qt,this._eventBus.fire(O),this._notifications.showNotification({text:`Play Execution`}),!1)},q.$inject=[`eventBus`,`tokenSimulationPalette`,`notifications`,`canvas`];var $t={__depends__:[Rt],__init__:[`pauseExecution`],pauseExecution:[`type`,q]},en=Ft();function J(e,t,n){this._eventBus=e,this._tokenSimulationPalette=t,this.canvasParent=n.getContainer().parentNode,this.isPaused=!1,this._init()}J.prototype._init=function(){this.paletteEntry=a(`
    <div class="bts-entry" title="Back to modeling">
      ${en}
    </div>
  `),d.bind(this.paletteEntry,`click`,this.toggle.bind(this)),this._tokenSimulationPalette.addEntry(this.paletteEntry,0)},J.prototype.toggle=function(){this._eventBus.fire(D,{active:!1})},J.$inject=[`eventBus`,`tokenSimulationPalette`,`canvas`];var tn={__init__:[`toggleModeling`],toggleModeling:[`type`,J]},nn=[[`Slow`,.5],[`Normal`,1],[`Fast`,2]];function Y(e,t,n){this._canvas=e,this._animation=t,this._eventBus=n,this._init(t.getAnimationSpeed()),n.on(D,e=>{e.active?s(this._container).remove(`hidden`):s(this._container).add(`hidden`)}),n.on(A,e=>{this.setActive(e.speed)})}Y.prototype.getToggleSpeed=function(e){return parseFloat(e.dataset.speed)},Y.prototype._init=function(e){this._container=a(`
    <div class="bts-set-animation-speed hidden">
      ${Nt()}
      <div class="bts-animation-speed-buttons">
        ${nn.map(([t,n],r)=>`
            <button title="Set animation speed = ${t}" data-speed="${n}" class="bts-animation-speed-button ${n===e?`active`:``}">
              ${Array.from({length:r+1}).map(()=>Pt()).join(``)}
            </button>
          `).join(``)}
      </div>
    </div>
  `),f.bind(this._container,`[data-speed]`,`click`,e=>{let t=e.delegateTarget,n=this.getToggleSpeed(t);this._animation.setAnimationSpeed(n)}),this._canvas.getContainer().appendChild(this._container)},Y.prototype.setActive=function(e){m(`[data-speed]`,this._container).forEach(t=>{let n=this.getToggleSpeed(t)===e;s(t)[n?`add`:`remove`](`active`)})},Y.$inject=[`canvas`,`animation`,`eventBus`];var rn={__depends__:[Ze,at,pt,_t,Rt,Wt,Jt,Yt,Xt,$t,tn,{__init__:[`setAnimationSpeed`],setAnimationSpeed:[`type`,Y]}],__init__:[`counterExampleVisualizer`],counterExampleVisualizer:[`type`,Re]},an=`PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIGhlaWdodD0iMjQiIHZpZXdCb3g9IjAgLTk2MCA5NjAgOTYwIiB3aWR0aD0iMjQiPg0KICA8cGF0aCBkPSJtNDAtMTIwIDQ0MC03NjAgNDQwIDc2MEg0MFptMTM4LTgwaDYwNEw0ODAtNzIwIDE3OC0yMDBabTMwMi00MHExNyAwIDI4LjUtMTEuNVQ1MjAtMjgwcTAtMTctMTEuNS0yOC41VDQ4MC0zMjBxLTE3IDAtMjguNSAxMS41VDQ0MC0yODBxMCAxNyAxMS41IDI4LjVUNDgwLTI0MFptLTQwLTEyMGg4MHYtMjAwaC04MHYyMDBabTQwLTEwMFoiIHN0cm9rZT0id2hpdGUiIGZpbGw9IndoaXRlIi8+DQo8L3N2Zz4NCg==`;function on(e,t,n,r){e.on(`analysis.done`,i),this._canvas=n,this._init();function i(e){if(e.unsupported_elements&&e.unsupported_elements.length>0){a(e);return}e.property_results.length!==Se.length&&o();for(let t of e.property_results)s(t)}function a(e){for(let n of e.unsupported_elements){let e=r.get(n);t.add(n,Ce,{position:{top:-45,left:e.width/2-17},html:`<div class="small-note tooltip warning-note">
                 <img alt="quick-fix" src="data:image/svg+xml;base64,${an}"/>
                 <span class="tooltipText">This element is currently unsupported by the analyzer.</span>
               </div>`})}o()}function o(){for(let e of Se){let t=document.getElementById(e);if(!t)continue;t.classList.remove(`violated`,`fulfilled`);let n=document.getElementById(`${e}-icon`);n&&(n.classList.add(`icon-question`),n.classList.remove(`icon-check`,`icon-xmark`,`fulfilled`,`violated`))}}function s(e){let t=document.getElementById(`${e.property}`),n=document.getElementById(`${e.property}-icon`);!t||!n||(e.fulfilled?(t.classList.remove(`violated`),t.classList.add(`fulfilled`),n.classList.remove(`icon-question`,`icon-xmark`,`violated`),n.classList.add(`icon-check`,`fulfilled`)):(t.classList.remove(`fulfilled`),t.classList.add(`violated`),n.classList.remove(`icon-question`,`icon-check`,`fulfilled`),n.classList.add(`icon-xmark`,`violated`)))}}on.prototype._init=function(){let e=a(`
    <div class="properties">
      <div id="Safeness">Synchronization</div>
      <div id="Safeness-icon" class="icon-question general-icon"></div>
      <div id="OptionToComplete">Guaranteed termination</div>
      <div id="OptionToComplete-icon" class="icon-question general-icon"></div>
      <div id="ProperCompletion">Unique end event execution</div>
      <div id="ProperCompletion-icon" class="icon-question general-icon"></div>
      <div id="NoDeadActivities">No dead activities</div>
      <div id="NoDeadActivities-icon" class="icon-question general-icon"></div>
    </div>
  `);this._canvas.getContainer().appendChild(e)},on.$inject=[`eventBus`,`overlays`,`canvas`,`elementRegistry`];var sn={__depends__:[me,Te,{__init__:[`propertiesSummary`],propertiesSummary:[`type`,on]},Le,rn]},X={taskSplit:`<?xml version="1.0" encoding="UTF-8"?>
<definitions xmlns:bpmndi="http://www.omg.org/spec/BPMN/20100524/DI" xmlns:dc="http://www.omg.org/spec/DD/20100524/DC" xmlns:di="http://www.omg.org/spec/DD/20100524/DI" xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL" id="definitions_793d1020-e08a-4f53-b128-c61827f033c9" targetNamespace="http://www.omg.org/spec/BPMN/20100524/MODEL" exporter="Camunda Modeler" exporterVersion="5.19.0">
  <process id="process_c648aa44-f99b-4cc0-8bba-9b1fafd7d01b" isExecutable="false">
    <startEvent id="FlowNode_1" name="start">
      <outgoing>Flow_0651tg3</outgoing>
    </startEvent>
    <task id="Activity_0x4hd7x" name="A">
      <incoming>Flow_0651tg3</incoming>
      <outgoing>Flow_1mt3e70</outgoing>
      <outgoing>Flow_0ouf0f2</outgoing>
    </task>
    <sequenceFlow id="Flow_0651tg3" sourceRef="FlowNode_1" targetRef="Activity_0x4hd7x" />
    <task id="Activity_0j1wpr5" name="B">
      <incoming>Flow_0ouf0f2</incoming>
      <outgoing>Flow_0h66a89</outgoing>
    </task>
    <sequenceFlow id="Flow_0ouf0f2" sourceRef="Activity_0x4hd7x" targetRef="Activity_0j1wpr5" />
    <exclusiveGateway id="Gateway_0kzt77d" name="e1">
      <incoming>Flow_0h66a89</incoming>
      <incoming>Flow_0rg1qmh</incoming>
      <outgoing>Flow_1hwufg1</outgoing>
    </exclusiveGateway>
    <sequenceFlow id="Flow_0h66a89" sourceRef="Activity_0j1wpr5" targetRef="Gateway_0kzt77d" />
    <endEvent id="Event_0omf2ig" name="end">
      <incoming>Flow_1hwufg1</incoming>
    </endEvent>
    <sequenceFlow id="Flow_1hwufg1" sourceRef="Gateway_0kzt77d" targetRef="Event_0omf2ig" />
    <task id="Activity_04cg2zp" name="C">
      <incoming>Flow_1mt3e70</incoming>
      <outgoing>Flow_0rg1qmh</outgoing>
    </task>
    <sequenceFlow id="Flow_1mt3e70" sourceRef="Activity_0x4hd7x" targetRef="Activity_04cg2zp" />
    <sequenceFlow id="Flow_0rg1qmh" sourceRef="Activity_04cg2zp" targetRef="Gateway_0kzt77d" />
  </process>
  <bpmndi:BPMNDiagram id="BPMNDiagram_40e0d24a-67a5-413e-aefc-dad265aaf73b">
    <bpmndi:BPMNPlane id="BPMNPlane_500f5dcb-d27a-4260-8cac-962e6eb35e01" bpmnElement="process_c648aa44-f99b-4cc0-8bba-9b1fafd7d01b">
      <bpmndi:BPMNShape id="BPMNShape_5604032a-8541-4e0f-9276-10d9de3c10df" bpmnElement="FlowNode_1">
        <dc:Bounds x="182" y="92" width="36" height="36" />
        <bpmndi:BPMNLabel>
          <dc:Bounds x="189" y="135" width="23" height="14" />
        </bpmndi:BPMNLabel>
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Activity_0x4hd7x_di" bpmnElement="Activity_0x4hd7x">
        <dc:Bounds x="270" y="70" width="100" height="80" />
        <bpmndi:BPMNLabel />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Activity_0j1wpr5_di" bpmnElement="Activity_0j1wpr5">
        <dc:Bounds x="430" y="70" width="100" height="80" />
        <bpmndi:BPMNLabel />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Gateway_0kzt77d_di" bpmnElement="Gateway_0kzt77d" isMarkerVisible="true">
        <dc:Bounds x="595" y="85" width="50" height="50" />
        <bpmndi:BPMNLabel>
          <dc:Bounds x="614" y="55" width="13" height="14" />
        </bpmndi:BPMNLabel>
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Event_0omf2ig_di" bpmnElement="Event_0omf2ig">
        <dc:Bounds x="712" y="92" width="36" height="36" />
        <bpmndi:BPMNLabel>
          <dc:Bounds x="721" y="135" width="19" height="14" />
        </bpmndi:BPMNLabel>
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Activity_04cg2zp_di" bpmnElement="Activity_04cg2zp">
        <dc:Bounds x="430" y="180" width="100" height="80" />
        <bpmndi:BPMNLabel />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNEdge id="Flow_0651tg3_di" bpmnElement="Flow_0651tg3">
        <di:waypoint x="218" y="110" />
        <di:waypoint x="270" y="110" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_0ouf0f2_di" bpmnElement="Flow_0ouf0f2">
        <di:waypoint x="370" y="110" />
        <di:waypoint x="430" y="110" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_0h66a89_di" bpmnElement="Flow_0h66a89">
        <di:waypoint x="530" y="110" />
        <di:waypoint x="595" y="110" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_1hwufg1_di" bpmnElement="Flow_1hwufg1">
        <di:waypoint x="645" y="110" />
        <di:waypoint x="712" y="110" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_1mt3e70_di" bpmnElement="Flow_1mt3e70">
        <di:waypoint x="320" y="150" />
        <di:waypoint x="320" y="220" />
        <di:waypoint x="430" y="220" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_0rg1qmh_di" bpmnElement="Flow_0rg1qmh">
        <di:waypoint x="530" y="220" />
        <di:waypoint x="620" y="220" />
        <di:waypoint x="620" y="135" />
      </bpmndi:BPMNEdge>
    </bpmndi:BPMNPlane>
  </bpmndi:BPMNDiagram>
</definitions>
`,taskMerge:`<?xml version="1.0" encoding="UTF-8"?>
<definitions xmlns:bpmndi="http://www.omg.org/spec/BPMN/20100524/DI" xmlns:dc="http://www.omg.org/spec/DD/20100524/DC" xmlns:di="http://www.omg.org/spec/DD/20100524/DI" xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL" id="definitions_793d1020-e08a-4f53-b128-c61827f033c9" targetNamespace="http://www.omg.org/spec/BPMN/20100524/MODEL" exporter="Camunda Modeler" exporterVersion="5.19.0">
  <process id="process_c648aa44-f99b-4cc0-8bba-9b1fafd7d01b" isExecutable="false">
    <startEvent id="FlowNode_1" name="start">
      <outgoing>Flow_18hry6m</outgoing>
    </startEvent>
    <sequenceFlow id="Flow_18hry6m" sourceRef="FlowNode_1" targetRef="Gateway_19m1xkh" />
    <parallelGateway id="Gateway_19m1xkh" name="p1">
      <incoming>Flow_18hry6m</incoming>
      <outgoing>Flow_1q22mal</outgoing>
      <outgoing>Flow_0wpnz5f</outgoing>
    </parallelGateway>
    <task id="Activity_15hugp3" name="C">
      <incoming>Flow_1qwb5uh</incoming>
      <incoming>Flow_083jwwy</incoming>
      <outgoing>Flow_0ymoh0m</outgoing>
    </task>
    <sequenceFlow id="Flow_1q22mal" sourceRef="Gateway_19m1xkh" targetRef="Activity_0btobfd" />
    <sequenceFlow id="Flow_0wpnz5f" sourceRef="Gateway_19m1xkh" targetRef="Activity_1p8bwbi" />
    <task id="Activity_0btobfd" name="A">
      <incoming>Flow_1q22mal</incoming>
      <outgoing>Flow_1qwb5uh</outgoing>
    </task>
    <sequenceFlow id="Flow_1qwb5uh" sourceRef="Activity_0btobfd" targetRef="Activity_15hugp3" />
    <task id="Activity_1p8bwbi" name="B">
      <incoming>Flow_0wpnz5f</incoming>
      <outgoing>Flow_083jwwy</outgoing>
    </task>
    <sequenceFlow id="Flow_083jwwy" sourceRef="Activity_1p8bwbi" targetRef="Activity_15hugp3" />
    <endEvent id="Event_0dpii84" name="end">
      <incoming>Flow_0ymoh0m</incoming>
    </endEvent>
    <sequenceFlow id="Flow_0ymoh0m" sourceRef="Activity_15hugp3" targetRef="Event_0dpii84" />
  </process>
  <bpmndi:BPMNDiagram id="BPMNDiagram_40e0d24a-67a5-413e-aefc-dad265aaf73b">
    <bpmndi:BPMNPlane id="BPMNPlane_500f5dcb-d27a-4260-8cac-962e6eb35e01" bpmnElement="process_c648aa44-f99b-4cc0-8bba-9b1fafd7d01b">
      <bpmndi:BPMNShape id="BPMNShape_5604032a-8541-4e0f-9276-10d9de3c10df" bpmnElement="FlowNode_1">
        <dc:Bounds x="152" y="162" width="36" height="36" />
        <bpmndi:BPMNLabel>
          <dc:Bounds x="159" y="205" width="23" height="14" />
        </bpmndi:BPMNLabel>
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Gateway_1l4k4l0_di" bpmnElement="Gateway_19m1xkh">
        <dc:Bounds x="245" y="155" width="50" height="50" />
        <bpmndi:BPMNLabel>
          <dc:Bounds x="264" y="125" width="13" height="14" />
        </bpmndi:BPMNLabel>
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Activity_15hugp3_di" bpmnElement="Activity_15hugp3">
        <dc:Bounds x="520" y="140" width="100" height="80" />
        <bpmndi:BPMNLabel />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Activity_0btobfd_di" bpmnElement="Activity_0btobfd">
        <dc:Bounds x="350" y="140" width="100" height="80" />
        <bpmndi:BPMNLabel />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Activity_1p8bwbi_di" bpmnElement="Activity_1p8bwbi">
        <dc:Bounds x="350" y="230" width="100" height="80" />
        <bpmndi:BPMNLabel />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Event_0dpii84_di" bpmnElement="Event_0dpii84">
        <dc:Bounds x="692" y="162" width="36" height="36" />
        <bpmndi:BPMNLabel>
          <dc:Bounds x="701" y="205" width="19" height="14" />
        </bpmndi:BPMNLabel>
      </bpmndi:BPMNShape>
      <bpmndi:BPMNEdge id="Flow_18hry6m_di" bpmnElement="Flow_18hry6m">
        <di:waypoint x="188" y="180" />
        <di:waypoint x="245" y="180" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_1q22mal_di" bpmnElement="Flow_1q22mal">
        <di:waypoint x="295" y="180" />
        <di:waypoint x="350" y="180" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_0wpnz5f_di" bpmnElement="Flow_0wpnz5f">
        <di:waypoint x="270" y="205" />
        <di:waypoint x="270" y="270" />
        <di:waypoint x="350" y="270" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_1qwb5uh_di" bpmnElement="Flow_1qwb5uh">
        <di:waypoint x="450" y="180" />
        <di:waypoint x="520" y="180" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_083jwwy_di" bpmnElement="Flow_083jwwy">
        <di:waypoint x="450" y="270" />
        <di:waypoint x="570" y="270" />
        <di:waypoint x="570" y="220" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_0ymoh0m_di" bpmnElement="Flow_0ymoh0m">
        <di:waypoint x="620" y="180" />
        <di:waypoint x="692" y="180" />
      </bpmndi:BPMNEdge>
    </bpmndi:BPMNPlane>
  </bpmndi:BPMNDiagram>
</definitions>
`,showcase:`<?xml version="1.0" encoding="UTF-8"?>
<definitions xmlns:bpmndi="http://www.omg.org/spec/BPMN/20100524/DI" xmlns:dc="http://www.omg.org/spec/DD/20100524/DC" xmlns:di="http://www.omg.org/spec/DD/20100524/DI" xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL" id="definitions_793d1020-e08a-4f53-b128-c61827f033c9" targetNamespace="http://www.omg.org/spec/BPMN/20100524/MODEL" exporter="Camunda Modeler" exporterVersion="5.19.0">
  <process id="process_c648aa44-f99b-4cc0-8bba-9b1fafd7d01b" isExecutable="false">
    <startEvent id="FlowNode_1">
      <outgoing>Flow_18hry6m</outgoing>
    </startEvent>
    <sequenceFlow id="Flow_18hry6m" sourceRef="FlowNode_1" targetRef="Gateway_19m1xkh" />
    <parallelGateway id="Gateway_19m1xkh" name="p1">
      <incoming>Flow_18hry6m</incoming>
      <outgoing>Flow_16uaxqz</outgoing>
      <outgoing>Flow_1efaah2</outgoing>
    </parallelGateway>
    <exclusiveGateway id="Gateway_0g0wslj" name="e1">
      <incoming>Flow_16uaxqz</incoming>
      <incoming>Flow_1efaah2</incoming>
      <outgoing>Flow_16otuif</outgoing>
    </exclusiveGateway>
    <sequenceFlow id="Flow_16uaxqz" sourceRef="Gateway_19m1xkh" targetRef="Gateway_0g0wslj" />
    <sequenceFlow id="Flow_1efaah2" sourceRef="Gateway_19m1xkh" targetRef="Gateway_0g0wslj" />
    <endEvent id="Event_0zpujef" name="end1">
      <incoming>Flow_0rj1ajz</incoming>
    </endEvent>
    <sequenceFlow id="Flow_16otuif" sourceRef="Gateway_0g0wslj" targetRef="Gateway_07q265d" />
    <exclusiveGateway id="Gateway_07q265d" name="e2">
      <incoming>Flow_16otuif</incoming>
      <outgoing>Flow_0rj1ajz</outgoing>
      <outgoing>Flow_1a7qrau</outgoing>
    </exclusiveGateway>
    <sequenceFlow id="Flow_0rj1ajz" sourceRef="Gateway_07q265d" targetRef="Event_0zpujef" />
    <sequenceFlow id="Flow_1a7qrau" sourceRef="Gateway_07q265d" targetRef="Gateway_1r5g9ue" />
    <parallelGateway id="Gateway_1r5g9ue" name="p2">
      <incoming>Flow_1a7qrau</incoming>
      <incoming>Flow_1ulbcjl</incoming>
      <outgoing>Flow_0noxem3</outgoing>
    </parallelGateway>
    <task id="Activity_1jsm4u8" name="A">
      <outgoing>Flow_1ulbcjl</outgoing>
    </task>
    <sequenceFlow id="Flow_1ulbcjl" sourceRef="Activity_1jsm4u8" targetRef="Gateway_1r5g9ue" />
    <endEvent id="Event_0ntel2a" name="end2">
      <incoming>Flow_0noxem3</incoming>
    </endEvent>
    <sequenceFlow id="Flow_0noxem3" sourceRef="Gateway_1r5g9ue" targetRef="Event_0ntel2a" />
  </process>
  <bpmndi:BPMNDiagram id="BPMNDiagram_40e0d24a-67a5-413e-aefc-dad265aaf73b">
    <bpmndi:BPMNPlane id="BPMNPlane_500f5dcb-d27a-4260-8cac-962e6eb35e01" bpmnElement="process_c648aa44-f99b-4cc0-8bba-9b1fafd7d01b">
      <bpmndi:BPMNShape id="BPMNShape_5604032a-8541-4e0f-9276-10d9de3c10df" bpmnElement="FlowNode_1">
        <dc:Bounds x="182" y="92" width="36" height="36" />
        <bpmndi:BPMNLabel>
          <dc:Bounds x="169" y="128" width="62" height="14" />
        </bpmndi:BPMNLabel>
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Gateway_1l4k4l0_di" bpmnElement="Gateway_19m1xkh">
        <dc:Bounds x="275" y="85" width="50" height="50" />
        <bpmndi:BPMNLabel>
          <dc:Bounds x="294" y="55" width="13" height="14" />
        </bpmndi:BPMNLabel>
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Gateway_0g0wslj_di" bpmnElement="Gateway_0g0wslj" isMarkerVisible="true">
        <dc:Bounds x="425" y="85" width="50" height="50" />
        <bpmndi:BPMNLabel>
          <dc:Bounds x="444" y="55" width="13" height="14" />
        </bpmndi:BPMNLabel>
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Event_0zpujef_di" bpmnElement="Event_0zpujef">
        <dc:Bounds x="652" y="92" width="36" height="36" />
        <bpmndi:BPMNLabel>
          <dc:Bounds x="697" y="103" width="25" height="14" />
        </bpmndi:BPMNLabel>
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Gateway_07q265d_di" bpmnElement="Gateway_07q265d" isMarkerVisible="true">
        <dc:Bounds x="535" y="85" width="50" height="50" />
        <bpmndi:BPMNLabel>
          <dc:Bounds x="554" y="55" width="13" height="14" />
        </bpmndi:BPMNLabel>
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Gateway_035ywgl_di" bpmnElement="Gateway_1r5g9ue">
        <dc:Bounds x="645" y="165" width="50" height="50" />
        <bpmndi:BPMNLabel>
          <dc:Bounds x="663" y="143" width="13" height="14" />
        </bpmndi:BPMNLabel>
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Activity_1jsm4u8_di" bpmnElement="Activity_1jsm4u8">
        <dc:Bounds x="460" y="260" width="100" height="80" />
        <bpmndi:BPMNLabel />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Event_0ntel2a_di" bpmnElement="Event_0ntel2a">
        <dc:Bounds x="762" y="172" width="36" height="36" />
        <bpmndi:BPMNLabel>
          <dc:Bounds x="768" y="215" width="25" height="14" />
        </bpmndi:BPMNLabel>
      </bpmndi:BPMNShape>
      <bpmndi:BPMNEdge id="Flow_18hry6m_di" bpmnElement="Flow_18hry6m">
        <di:waypoint x="218" y="110" />
        <di:waypoint x="275" y="110" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_16uaxqz_di" bpmnElement="Flow_16uaxqz">
        <di:waypoint x="325" y="110" />
        <di:waypoint x="425" y="110" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_1efaah2_di" bpmnElement="Flow_1efaah2">
        <di:waypoint x="300" y="135" />
        <di:waypoint x="300" y="190" />
        <di:waypoint x="450" y="190" />
        <di:waypoint x="450" y="135" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_16otuif_di" bpmnElement="Flow_16otuif">
        <di:waypoint x="475" y="110" />
        <di:waypoint x="535" y="110" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_0rj1ajz_di" bpmnElement="Flow_0rj1ajz">
        <di:waypoint x="585" y="110" />
        <di:waypoint x="652" y="110" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_1a7qrau_di" bpmnElement="Flow_1a7qrau">
        <di:waypoint x="560" y="135" />
        <di:waypoint x="560" y="190" />
        <di:waypoint x="645" y="190" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_1ulbcjl_di" bpmnElement="Flow_1ulbcjl">
        <di:waypoint x="560" y="300" />
        <di:waypoint x="670" y="300" />
        <di:waypoint x="670" y="215" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_0noxem3_di" bpmnElement="Flow_0noxem3">
        <di:waypoint x="695" y="190" />
        <di:waypoint x="762" y="190" />
      </bpmndi:BPMNEdge>
    </bpmndi:BPMNPlane>
  </bpmndi:BPMNDiagram>
</definitions>
`,unsafeGateways:`<?xml version="1.0" encoding="UTF-8"?>
<definitions xmlns:bpmndi="http://www.omg.org/spec/BPMN/20100524/DI" xmlns:dc="http://www.omg.org/spec/DD/20100524/DC" xmlns:di="http://www.omg.org/spec/DD/20100524/DI" xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL" id="definitions_793d1020-e08a-4f53-b128-c61827f033c9" targetNamespace="http://www.omg.org/spec/BPMN/20100524/MODEL" exporter="Camunda Modeler" exporterVersion="5.19.0">
  <process id="process_c648aa44-f99b-4cc0-8bba-9b1fafd7d01b" isExecutable="false">
    <startEvent id="FlowNode_1" name="start">
      <outgoing>Flow_18hry6m</outgoing>
    </startEvent>
    <sequenceFlow id="Flow_18hry6m" sourceRef="FlowNode_1" targetRef="Gateway_19m1xkh" />
    <task id="Activity_1k9hlq7" name="A">
      <incoming>Flow_0k243p1</incoming>
      <outgoing>Flow_0msbshj</outgoing>
    </task>
    <sequenceFlow id="Flow_0k243p1" sourceRef="Gateway_19m1xkh" targetRef="Activity_1k9hlq7" />
    <task id="Activity_0b89un0" name="B">
      <incoming>Flow_1q12yo1</incoming>
      <outgoing>Flow_171s3u9</outgoing>
    </task>
    <sequenceFlow id="Flow_1q12yo1" sourceRef="Gateway_19m1xkh" targetRef="Activity_0b89un0" />
    <sequenceFlow id="Flow_0msbshj" sourceRef="Activity_1k9hlq7" targetRef="Gateway_17yykq8" />
    <sequenceFlow id="Flow_171s3u9" sourceRef="Activity_0b89un0" targetRef="Gateway_17yykq8" />
    <exclusiveGateway id="Gateway_17yykq8" name="e1">
      <incoming>Flow_171s3u9</incoming>
      <incoming>Flow_0msbshj</incoming>
      <outgoing>Flow_0yhwwde</outgoing>
    </exclusiveGateway>
    <endEvent id="Event_0er1rer" name="end">
      <incoming>Flow_0yhwwde</incoming>
    </endEvent>
    <sequenceFlow id="Flow_0yhwwde" sourceRef="Gateway_17yykq8" targetRef="Event_0er1rer" />
    <parallelGateway id="Gateway_19m1xkh" name="p1">
      <incoming>Flow_18hry6m</incoming>
      <outgoing>Flow_1q12yo1</outgoing>
      <outgoing>Flow_0k243p1</outgoing>
    </parallelGateway>
  </process>
  <bpmndi:BPMNDiagram id="BPMNDiagram_40e0d24a-67a5-413e-aefc-dad265aaf73b">
    <bpmndi:BPMNPlane id="BPMNPlane_500f5dcb-d27a-4260-8cac-962e6eb35e01" bpmnElement="process_c648aa44-f99b-4cc0-8bba-9b1fafd7d01b">
      <bpmndi:BPMNShape id="BPMNShape_5604032a-8541-4e0f-9276-10d9de3c10df" bpmnElement="FlowNode_1">
        <dc:Bounds x="182" y="92" width="36" height="36" />
        <bpmndi:BPMNLabel>
          <dc:Bounds x="189" y="135" width="23" height="14" />
        </bpmndi:BPMNLabel>
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Activity_1k9hlq7_di" bpmnElement="Activity_1k9hlq7">
        <dc:Bounds x="390" y="70" width="100" height="80" />
        <bpmndi:BPMNLabel />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Activity_0b89un0_di" bpmnElement="Activity_0b89un0">
        <dc:Bounds x="390" y="180" width="100" height="80" />
        <bpmndi:BPMNLabel />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Gateway_17yykq8_di" bpmnElement="Gateway_17yykq8" isMarkerVisible="true">
        <dc:Bounds x="555" y="85" width="50" height="50" />
        <bpmndi:BPMNLabel>
          <dc:Bounds x="574" y="55" width="13" height="14" />
        </bpmndi:BPMNLabel>
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Event_0er1rer_di" bpmnElement="Event_0er1rer">
        <dc:Bounds x="672" y="92" width="36" height="36" />
        <bpmndi:BPMNLabel>
          <dc:Bounds x="681" y="135" width="19" height="14" />
        </bpmndi:BPMNLabel>
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Gateway_0ovwr6j_di" bpmnElement="Gateway_19m1xkh">
        <dc:Bounds x="275" y="85" width="50" height="50" />
        <bpmndi:BPMNLabel>
          <dc:Bounds x="294" y="55" width="13" height="14" />
        </bpmndi:BPMNLabel>
      </bpmndi:BPMNShape>
      <bpmndi:BPMNEdge id="Flow_18hry6m_di" bpmnElement="Flow_18hry6m">
        <di:waypoint x="218" y="110" />
        <di:waypoint x="275" y="110" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_0k243p1_di" bpmnElement="Flow_0k243p1">
        <di:waypoint x="325" y="110" />
        <di:waypoint x="390" y="110" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_1q12yo1_di" bpmnElement="Flow_1q12yo1">
        <di:waypoint x="300" y="135" />
        <di:waypoint x="300" y="220" />
        <di:waypoint x="390" y="220" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_0msbshj_di" bpmnElement="Flow_0msbshj">
        <di:waypoint x="490" y="110" />
        <di:waypoint x="555" y="110" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_171s3u9_di" bpmnElement="Flow_171s3u9">
        <di:waypoint x="490" y="220" />
        <di:waypoint x="580" y="220" />
        <di:waypoint x="580" y="135" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_0yhwwde_di" bpmnElement="Flow_0yhwwde">
        <di:waypoint x="605" y="110" />
        <di:waypoint x="672" y="110" />
      </bpmndi:BPMNEdge>
    </bpmndi:BPMNPlane>
  </bpmndi:BPMNDiagram>
</definitions>
`,reusedEndEvent:`<?xml version="1.0" encoding="UTF-8"?>
<definitions xmlns:bpmndi="http://www.omg.org/spec/BPMN/20100524/DI" xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL" xmlns:dc="http://www.omg.org/spec/DD/20100524/DC" xmlns:di="http://www.omg.org/spec/DD/20100524/DI" id="definitions_793d1020-e08a-4f53-b128-c61827f033c9" targetNamespace="http://www.omg.org/spec/BPMN/20100524/MODEL" exporter="Camunda Modeler" exporterVersion="5.19.0">
  <process id="process_c648aa44-f99b-4cc0-8bba-9b1fafd7d01b" isExecutable="false">
    <startEvent id="Event_0h3sh8n" name="start">
      <outgoing>Flow_0i1i4f2</outgoing>
    </startEvent>
    <sequenceFlow id="Flow_0i1i4f2" sourceRef="Event_0h3sh8n" targetRef="Gateway_05483qp" />
    <parallelGateway id="Gateway_05483qp" name="p1">
      <incoming>Flow_0i1i4f2</incoming>
      <outgoing>Flow_0wntnjd</outgoing>
      <outgoing>Flow_1ashgqg</outgoing>
    </parallelGateway>
    <task id="Activity_08tyv4v" name="A">
      <incoming>Flow_0wntnjd</incoming>
      <outgoing>Flow_0x778wb</outgoing>
    </task>
    <sequenceFlow id="Flow_0wntnjd" sourceRef="Gateway_05483qp" targetRef="Activity_08tyv4v" />
    <task id="Activity_1j6jezt" name="B">
      <incoming>Flow_1ashgqg</incoming>
      <outgoing>Flow_0a5feas</outgoing>
    </task>
    <sequenceFlow id="Flow_1ashgqg" sourceRef="Gateway_05483qp" targetRef="Activity_1j6jezt" />
    <endEvent id="Event_081kyk3" name="end">
      <incoming>Flow_0x778wb</incoming>
      <incoming>Flow_0a5feas</incoming>
    </endEvent>
    <sequenceFlow id="Flow_0x778wb" name="A" sourceRef="Activity_08tyv4v" targetRef="Event_081kyk3" />
    <sequenceFlow id="Flow_0a5feas" name="B" sourceRef="Activity_1j6jezt" targetRef="Event_081kyk3" />
  </process>
  <bpmndi:BPMNDiagram id="BPMNDiagram_40e0d24a-67a5-413e-aefc-dad265aaf73b">
    <bpmndi:BPMNPlane id="BPMNPlane_500f5dcb-d27a-4260-8cac-962e6eb35e01" bpmnElement="process_c648aa44-f99b-4cc0-8bba-9b1fafd7d01b">
      <bpmndi:BPMNShape id="Event_0h3sh8n_di" bpmnElement="Event_0h3sh8n">
        <dc:Bounds x="152" y="122" width="36" height="36" />
        <bpmndi:BPMNLabel>
          <dc:Bounds x="159" y="165" width="23" height="14" />
        </bpmndi:BPMNLabel>
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Gateway_10ktaec_di" bpmnElement="Gateway_05483qp">
        <dc:Bounds x="245" y="115" width="50" height="50" />
        <bpmndi:BPMNLabel>
          <dc:Bounds x="264" y="85" width="13" height="14" />
        </bpmndi:BPMNLabel>
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Activity_08tyv4v_di" bpmnElement="Activity_08tyv4v">
        <dc:Bounds x="360" y="100" width="100" height="80" />
        <bpmndi:BPMNLabel />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Activity_1j6jezt_di" bpmnElement="Activity_1j6jezt">
        <dc:Bounds x="360" y="210" width="100" height="80" />
        <bpmndi:BPMNLabel />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Event_081kyk3_di" bpmnElement="Event_081kyk3">
        <dc:Bounds x="532" y="122" width="36" height="36" />
        <bpmndi:BPMNLabel>
          <dc:Bounds x="541" y="92" width="19" height="14" />
        </bpmndi:BPMNLabel>
      </bpmndi:BPMNShape>
      <bpmndi:BPMNEdge id="Flow_0i1i4f2_di" bpmnElement="Flow_0i1i4f2">
        <di:waypoint x="188" y="140" />
        <di:waypoint x="245" y="140" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_0wntnjd_di" bpmnElement="Flow_0wntnjd">
        <di:waypoint x="295" y="140" />
        <di:waypoint x="360" y="140" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_1ashgqg_di" bpmnElement="Flow_1ashgqg">
        <di:waypoint x="270" y="165" />
        <di:waypoint x="270" y="250" />
        <di:waypoint x="360" y="250" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_0x778wb_di" bpmnElement="Flow_0x778wb">
        <di:waypoint x="460" y="140" />
        <di:waypoint x="532" y="140" />
        <bpmndi:BPMNLabel>
          <dc:Bounds x="506" y="122" width="7" height="14" />
        </bpmndi:BPMNLabel>
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_0a5feas_di" bpmnElement="Flow_0a5feas">
        <di:waypoint x="460" y="250" />
        <di:waypoint x="550" y="250" />
        <di:waypoint x="550" y="158" />
        <bpmndi:BPMNLabel>
          <dc:Bounds x="506" y="232" width="8" height="14" />
        </bpmndi:BPMNLabel>
      </bpmndi:BPMNEdge>
    </bpmndi:BPMNPlane>
  </bpmndi:BPMNDiagram>
</definitions>
`,stuck:`<?xml version="1.0" encoding="UTF-8"?>
<definitions xmlns:bpmndi="http://www.omg.org/spec/BPMN/20100524/DI" xmlns:dc="http://www.omg.org/spec/DD/20100524/DC" xmlns:di="http://www.omg.org/spec/DD/20100524/DI" xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL" id="definitions_793d1020-e08a-4f53-b128-c61827f033c9" targetNamespace="http://www.omg.org/spec/BPMN/20100524/MODEL" exporter="Camunda Modeler" exporterVersion="5.19.0">
  <process id="process_c648aa44-f99b-4cc0-8bba-9b1fafd7d01b" isExecutable="false">
    <startEvent id="FlowNode_1" name="start">
      <outgoing>Flow_18hry6m</outgoing>
    </startEvent>
    <sequenceFlow id="Flow_18hry6m" sourceRef="FlowNode_1" targetRef="Gateway_19m1xkh" />
    <task id="Activity_1k9hlq7" name="A">
      <incoming>Flow_0k243p1</incoming>
      <outgoing>Flow_0msbshj</outgoing>
    </task>
    <sequenceFlow id="Flow_0k243p1" sourceRef="Gateway_19m1xkh" targetRef="Activity_1k9hlq7" />
    <task id="Activity_0b89un0" name="B">
      <incoming>Flow_1q12yo1</incoming>
      <outgoing>Flow_171s3u9</outgoing>
    </task>
    <sequenceFlow id="Flow_1q12yo1" sourceRef="Gateway_19m1xkh" targetRef="Activity_0b89un0" />
    <sequenceFlow id="Flow_0msbshj" sourceRef="Activity_1k9hlq7" targetRef="Gateway_17yykq8" />
    <sequenceFlow id="Flow_171s3u9" sourceRef="Activity_0b89un0" targetRef="Gateway_17yykq8" />
    <task id="Activity_0euzcpq" name="C">
      <incoming>Flow_0z37bdf</incoming>
      <outgoing>Flow_1c24te8</outgoing>
    </task>
    <sequenceFlow id="Flow_0z37bdf" sourceRef="Gateway_17yykq8" targetRef="Activity_0euzcpq" />
    <endEvent id="Event_0wi1fn1" name="end">
      <incoming>Flow_1c24te8</incoming>
    </endEvent>
    <sequenceFlow id="Flow_1c24te8" sourceRef="Activity_0euzcpq" targetRef="Event_0wi1fn1" />
    <exclusiveGateway id="Gateway_19m1xkh" name="e1">
      <incoming>Flow_18hry6m</incoming>
      <outgoing>Flow_0k243p1</outgoing>
      <outgoing>Flow_1q12yo1</outgoing>
    </exclusiveGateway>
    <parallelGateway id="Gateway_17yykq8" name="p1">
      <incoming>Flow_0msbshj</incoming>
      <incoming>Flow_171s3u9</incoming>
      <outgoing>Flow_0z37bdf</outgoing>
    </parallelGateway>
  </process>
  <bpmndi:BPMNDiagram id="BPMNDiagram_40e0d24a-67a5-413e-aefc-dad265aaf73b">
    <bpmndi:BPMNPlane id="BPMNPlane_500f5dcb-d27a-4260-8cac-962e6eb35e01" bpmnElement="process_c648aa44-f99b-4cc0-8bba-9b1fafd7d01b">
      <bpmndi:BPMNShape id="BPMNShape_5604032a-8541-4e0f-9276-10d9de3c10df" bpmnElement="FlowNode_1">
        <dc:Bounds x="182" y="92" width="36" height="36" />
        <bpmndi:BPMNLabel>
          <dc:Bounds x="189" y="135" width="23" height="14" />
        </bpmndi:BPMNLabel>
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Activity_1k9hlq7_di" bpmnElement="Activity_1k9hlq7">
        <dc:Bounds x="390" y="70" width="100" height="80" />
        <bpmndi:BPMNLabel />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Activity_0b89un0_di" bpmnElement="Activity_0b89un0">
        <dc:Bounds x="390" y="180" width="100" height="80" />
        <bpmndi:BPMNLabel />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Activity_0euzcpq_di" bpmnElement="Activity_0euzcpq">
        <dc:Bounds x="670" y="70" width="100" height="80" />
        <bpmndi:BPMNLabel />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Event_0wi1fn1_di" bpmnElement="Event_0wi1fn1">
        <dc:Bounds x="842" y="92" width="36" height="36" />
        <bpmndi:BPMNLabel>
          <dc:Bounds x="851" y="135" width="19" height="14" />
        </bpmndi:BPMNLabel>
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Gateway_0hq73ub_di" bpmnElement="Gateway_19m1xkh" isMarkerVisible="true">
        <dc:Bounds x="275" y="85" width="50" height="50" />
        <bpmndi:BPMNLabel>
          <dc:Bounds x="294" y="55" width="13" height="14" />
        </bpmndi:BPMNLabel>
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Gateway_1errs79_di" bpmnElement="Gateway_17yykq8">
        <dc:Bounds x="555" y="85" width="50" height="50" />
        <bpmndi:BPMNLabel>
          <dc:Bounds x="574" y="55" width="13" height="14" />
        </bpmndi:BPMNLabel>
      </bpmndi:BPMNShape>
      <bpmndi:BPMNEdge id="Flow_18hry6m_di" bpmnElement="Flow_18hry6m">
        <di:waypoint x="218" y="110" />
        <di:waypoint x="275" y="110" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_0k243p1_di" bpmnElement="Flow_0k243p1">
        <di:waypoint x="325" y="110" />
        <di:waypoint x="390" y="110" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_1q12yo1_di" bpmnElement="Flow_1q12yo1">
        <di:waypoint x="300" y="135" />
        <di:waypoint x="300" y="220" />
        <di:waypoint x="390" y="220" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_0msbshj_di" bpmnElement="Flow_0msbshj">
        <di:waypoint x="490" y="110" />
        <di:waypoint x="555" y="110" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_171s3u9_di" bpmnElement="Flow_171s3u9">
        <di:waypoint x="490" y="220" />
        <di:waypoint x="580" y="220" />
        <di:waypoint x="580" y="135" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_0z37bdf_di" bpmnElement="Flow_0z37bdf">
        <di:waypoint x="605" y="110" />
        <di:waypoint x="670" y="110" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_1c24te8_di" bpmnElement="Flow_1c24te8">
        <di:waypoint x="770" y="110" />
        <di:waypoint x="842" y="110" />
      </bpmndi:BPMNEdge>
    </bpmndi:BPMNPlane>
  </bpmndi:BPMNDiagram>
</definitions>
`,deadActivity:`<?xml version="1.0" encoding="UTF-8"?>
<definitions xmlns:bpmndi="http://www.omg.org/spec/BPMN/20100524/DI" xmlns:dc="http://www.omg.org/spec/DD/20100524/DC" xmlns:di="http://www.omg.org/spec/DD/20100524/DI" xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL" id="definitions_793d1020-e08a-4f53-b128-c61827f033c9" targetNamespace="http://www.omg.org/spec/BPMN/20100524/MODEL" exporter="Camunda Modeler" exporterVersion="5.19.0">
  <process id="process_c648aa44-f99b-4cc0-8bba-9b1fafd7d01b" isExecutable="false">
    <startEvent id="Event_0vaqaf3" name="start">
      <outgoing>Flow_0n9vknp</outgoing>
    </startEvent>
    <exclusiveGateway id="Gateway_0jozlxk" name="e1">
      <incoming>Flow_0n9vknp</incoming>
      <outgoing>Flow_04ju504</outgoing>
    </exclusiveGateway>
    <sequenceFlow id="Flow_0n9vknp" sourceRef="Event_0vaqaf3" targetRef="Gateway_0jozlxk" />
    <task id="Activity_0v4txpp" name="A">
      <incoming>Flow_04ju504</incoming>
      <outgoing>Flow_17gwp5i</outgoing>
    </task>
    <sequenceFlow id="Flow_04ju504" sourceRef="Gateway_0jozlxk" targetRef="Activity_0v4txpp" />
    <exclusiveGateway id="Gateway_1ygqqy0" name="e1">
      <incoming>Flow_17gwp5i</incoming>
      <incoming>Flow_1iru2dn</incoming>
      <outgoing>Flow_1sj90jw</outgoing>
    </exclusiveGateway>
    <sequenceFlow id="Flow_17gwp5i" sourceRef="Activity_0v4txpp" targetRef="Gateway_1ygqqy0" />
    <endEvent id="Event_142iq0a" name="end">
      <incoming>Flow_1sj90jw</incoming>
    </endEvent>
    <sequenceFlow id="Flow_1sj90jw" sourceRef="Gateway_1ygqqy0" targetRef="Event_142iq0a" />
    <task id="Activity_0ahojhq" name="B">
      <outgoing>Flow_1iru2dn</outgoing>
    </task>
    <sequenceFlow id="Flow_1iru2dn" sourceRef="Activity_0ahojhq" targetRef="Gateway_1ygqqy0" />
  </process>
  <bpmndi:BPMNDiagram id="BPMNDiagram_40e0d24a-67a5-413e-aefc-dad265aaf73b">
    <bpmndi:BPMNPlane id="BPMNPlane_500f5dcb-d27a-4260-8cac-962e6eb35e01" bpmnElement="process_c648aa44-f99b-4cc0-8bba-9b1fafd7d01b">
      <bpmndi:BPMNShape id="Event_0vaqaf3_di" bpmnElement="Event_0vaqaf3">
        <dc:Bounds x="152" y="122" width="36" height="36" />
        <bpmndi:BPMNLabel>
          <dc:Bounds x="159" y="165" width="23" height="14" />
        </bpmndi:BPMNLabel>
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Gateway_0jozlxk_di" bpmnElement="Gateway_0jozlxk" isMarkerVisible="true">
        <dc:Bounds x="245" y="115" width="50" height="50" />
        <bpmndi:BPMNLabel>
          <dc:Bounds x="264" y="172" width="13" height="14" />
        </bpmndi:BPMNLabel>
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Activity_0v4txpp_di" bpmnElement="Activity_0v4txpp">
        <dc:Bounds x="360" y="100" width="100" height="80" />
        <bpmndi:BPMNLabel />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Gateway_1ygqqy0_di" bpmnElement="Gateway_1ygqqy0" isMarkerVisible="true">
        <dc:Bounds x="525" y="115" width="50" height="50" />
        <bpmndi:BPMNLabel>
          <dc:Bounds x="544" y="85" width="13" height="14" />
        </bpmndi:BPMNLabel>
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Event_142iq0a_di" bpmnElement="Event_142iq0a">
        <dc:Bounds x="642" y="122" width="36" height="36" />
        <bpmndi:BPMNLabel>
          <dc:Bounds x="651" y="165" width="19" height="14" />
        </bpmndi:BPMNLabel>
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Activity_0ahojhq_di" bpmnElement="Activity_0ahojhq">
        <dc:Bounds x="360" y="210" width="100" height="80" />
        <bpmndi:BPMNLabel />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNEdge id="Flow_0n9vknp_di" bpmnElement="Flow_0n9vknp">
        <di:waypoint x="188" y="140" />
        <di:waypoint x="245" y="140" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_04ju504_di" bpmnElement="Flow_04ju504">
        <di:waypoint x="295" y="140" />
        <di:waypoint x="360" y="140" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_17gwp5i_di" bpmnElement="Flow_17gwp5i">
        <di:waypoint x="460" y="140" />
        <di:waypoint x="525" y="140" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_1sj90jw_di" bpmnElement="Flow_1sj90jw">
        <di:waypoint x="575" y="140" />
        <di:waypoint x="642" y="140" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_1iru2dn_di" bpmnElement="Flow_1iru2dn">
        <di:waypoint x="460" y="250" />
        <di:waypoint x="550" y="250" />
        <di:waypoint x="550" y="165" />
      </bpmndi:BPMNEdge>
    </bpmndi:BPMNPlane>
  </bpmndi:BPMNDiagram>
</definitions>
`,poolsWithMessageFlows:`<?xml version="1.0" encoding="UTF-8"?>
<bpmn:definitions xmlns:bpmn="http://www.omg.org/spec/BPMN/20100524/MODEL" xmlns:bpmndi="http://www.omg.org/spec/BPMN/20100524/DI" xmlns:dc="http://www.omg.org/spec/DD/20100524/DC" xmlns:di="http://www.omg.org/spec/DD/20100524/DI" xmlns:camunda="http://camunda.org/schema/1.0/bpmn" id="Definitions_1" targetNamespace="http://bpmn.io/schema/bpmn" exporter="Camunda Modeler" exporterVersion="5.7.0" camunda:diagramRelationId="c0ee7b3d-cbff-4320-b2f5-c9f17011829d">
  <bpmn:collaboration id="Collaboration_0rxoef1">
    <bpmn:participant id="p1" name="p1" processRef="p1_process" />
    <bpmn:participant id="p2" name="p2" processRef="p2_process" />
    <bpmn:participant id="p3" name="p3" processRef="p3_process" />
    <bpmn:messageFlow id="Flow_0mj6zqd" sourceRef="sendEvent" targetRef="startP2" />
    <bpmn:messageFlow id="Flow_1l5tj5s" sourceRef="SendTask" targetRef="receiveEvent" />
    <bpmn:messageFlow id="Flow_1teo7b1" sourceRef="endP1" targetRef="ReceiveTask" />
    <bpmn:messageFlow id="Flow_1k7qfrf" sourceRef="endP2" targetRef="Event_0wpufyp" />
    <bpmn:messageFlow id="Flow_1ukddvo" sourceRef="endP2" targetRef="Event_0acx3tz" />
  </bpmn:collaboration>
  <bpmn:process id="p1_process" isExecutable="true">
    <bpmn:startEvent id="startP1" name="startP1">
      <bpmn:outgoing>Flow_04pas1n</bpmn:outgoing>
    </bpmn:startEvent>
    <bpmn:intermediateThrowEvent id="sendEvent" name="sendEvent">
      <bpmn:incoming>Flow_04pas1n</bpmn:incoming>
      <bpmn:outgoing>Flow_1j9q5v2</bpmn:outgoing>
      <bpmn:messageEventDefinition id="MessageEventDefinition_1v4eir7" />
    </bpmn:intermediateThrowEvent>
    <bpmn:sendTask id="SendTask" name="SendTask">
      <bpmn:incoming>Flow_1j9q5v2</bpmn:incoming>
      <bpmn:outgoing>Flow_11etsb2</bpmn:outgoing>
    </bpmn:sendTask>
    <bpmn:endEvent id="endP1" name="endP1">
      <bpmn:incoming>Flow_11etsb2</bpmn:incoming>
      <bpmn:messageEventDefinition id="MessageEventDefinition_0cvzeof" />
    </bpmn:endEvent>
    <bpmn:sequenceFlow id="Flow_04pas1n" sourceRef="startP1" targetRef="sendEvent" />
    <bpmn:sequenceFlow id="Flow_1j9q5v2" sourceRef="sendEvent" targetRef="SendTask" />
    <bpmn:sequenceFlow id="Flow_11etsb2" sourceRef="SendTask" targetRef="endP1" />
  </bpmn:process>
  <bpmn:process id="p2_process" isExecutable="false">
    <bpmn:startEvent id="startP2" name="startP2">
      <bpmn:outgoing>Flow_1osubad</bpmn:outgoing>
      <bpmn:messageEventDefinition id="MessageEventDefinition_173pkcr" />
    </bpmn:startEvent>
    <bpmn:intermediateCatchEvent id="receiveEvent" name="receiveEvent">
      <bpmn:incoming>Flow_1osubad</bpmn:incoming>
      <bpmn:outgoing>Flow_0vrh0u6</bpmn:outgoing>
      <bpmn:messageEventDefinition id="MessageEventDefinition_0v3fw53" />
    </bpmn:intermediateCatchEvent>
    <bpmn:receiveTask id="ReceiveTask" name="ReceiveTask">
      <bpmn:incoming>Flow_0vrh0u6</bpmn:incoming>
      <bpmn:outgoing>Flow_1dm46gf</bpmn:outgoing>
    </bpmn:receiveTask>
    <bpmn:sequenceFlow id="Flow_1osubad" sourceRef="startP2" targetRef="receiveEvent" />
    <bpmn:sequenceFlow id="Flow_0vrh0u6" sourceRef="receiveEvent" targetRef="ReceiveTask" />
    <bpmn:sequenceFlow id="Flow_1dm46gf" sourceRef="ReceiveTask" targetRef="endP2" />
    <bpmn:endEvent id="endP2" name="endP2">
      <bpmn:incoming>Flow_1dm46gf</bpmn:incoming>
      <bpmn:messageEventDefinition id="MessageEventDefinition_04w7jid" />
    </bpmn:endEvent>
  </bpmn:process>
  <bpmn:process id="p3_process" isExecutable="false">
    <bpmn:startEvent id="Event_0kdhzc1" name="startP3">
      <bpmn:outgoing>Flow_1cnujy1</bpmn:outgoing>
    </bpmn:startEvent>
    <bpmn:sequenceFlow id="Flow_1cnujy1" sourceRef="Event_0kdhzc1" targetRef="Gateway_1pgg1e7" />
    <bpmn:eventBasedGateway id="Gateway_1pgg1e7">
      <bpmn:incoming>Flow_1cnujy1</bpmn:incoming>
      <bpmn:outgoing>Flow_06n5qhb</bpmn:outgoing>
      <bpmn:outgoing>Flow_1qh0vc5</bpmn:outgoing>
    </bpmn:eventBasedGateway>
    <bpmn:intermediateCatchEvent id="Event_0wpufyp" name="mice">
      <bpmn:incoming>Flow_06n5qhb</bpmn:incoming>
      <bpmn:outgoing>Flow_1i5w9wa</bpmn:outgoing>
      <bpmn:messageEventDefinition id="MessageEventDefinition_0cygvbl" />
    </bpmn:intermediateCatchEvent>
    <bpmn:sequenceFlow id="Flow_06n5qhb" sourceRef="Gateway_1pgg1e7" targetRef="Event_0wpufyp" />
    <bpmn:sequenceFlow id="Flow_1i5w9wa" sourceRef="Event_0wpufyp" targetRef="Event_1ek995l" />
    <bpmn:endEvent id="Event_1ek995l" name="endp3">
      <bpmn:incoming>Flow_1i5w9wa</bpmn:incoming>
    </bpmn:endEvent>
    <bpmn:intermediateCatchEvent id="Event_0acx3tz">
      <bpmn:incoming>Flow_1qh0vc5</bpmn:incoming>
      <bpmn:outgoing>Flow_1qw78et</bpmn:outgoing>
      <bpmn:messageEventDefinition id="MessageEventDefinition_1dzcj85" />
    </bpmn:intermediateCatchEvent>
    <bpmn:sequenceFlow id="Flow_1qh0vc5" sourceRef="Gateway_1pgg1e7" targetRef="Event_0acx3tz" />
    <bpmn:sequenceFlow id="Flow_1qw78et" sourceRef="Event_0acx3tz" targetRef="Gateway_0ff8w9b" />
    <bpmn:exclusiveGateway id="Gateway_1gibfd6">
      <bpmn:incoming>Flow_03hf6ha</bpmn:incoming>
      <bpmn:incoming>Flow_0huzdij</bpmn:incoming>
      <bpmn:outgoing>Flow_1b3pv18</bpmn:outgoing>
    </bpmn:exclusiveGateway>
    <bpmn:sequenceFlow id="Flow_03hf6ha" sourceRef="Gateway_0ff8w9b" targetRef="Gateway_1gibfd6" />
    <bpmn:endEvent id="Event_0w1ysb8">
      <bpmn:incoming>Flow_1b3pv18</bpmn:incoming>
    </bpmn:endEvent>
    <bpmn:sequenceFlow id="Flow_1b3pv18" sourceRef="Gateway_1gibfd6" targetRef="Event_0w1ysb8" />
    <bpmn:sequenceFlow id="Flow_0huzdij" sourceRef="Gateway_0ff8w9b" targetRef="Gateway_1gibfd6" />
    <bpmn:parallelGateway id="Gateway_0ff8w9b">
      <bpmn:incoming>Flow_1qw78et</bpmn:incoming>
      <bpmn:outgoing>Flow_03hf6ha</bpmn:outgoing>
      <bpmn:outgoing>Flow_0huzdij</bpmn:outgoing>
    </bpmn:parallelGateway>
  </bpmn:process>
  <bpmndi:BPMNDiagram id="BPMNDiagram_1">
    <bpmndi:BPMNPlane id="BPMNPlane_1" bpmnElement="Collaboration_0rxoef1">
      <bpmndi:BPMNShape id="Participant_1srjs4o_di" bpmnElement="p1" isHorizontal="true">
        <dc:Bounds x="160" y="78" width="520" height="142" />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="_BPMNShape_StartEvent_2" bpmnElement="startP1">
        <dc:Bounds x="210" y="132" width="36" height="36" />
        <bpmndi:BPMNLabel>
          <dc:Bounds x="211" y="175" width="35" height="14" />
        </bpmndi:BPMNLabel>
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Event_0wa7eq4_di" bpmnElement="sendEvent">
        <dc:Bounds x="302" y="132" width="36" height="36" />
        <bpmndi:BPMNLabel>
          <dc:Bounds x="293" y="113" width="53" height="14" />
        </bpmndi:BPMNLabel>
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Activity_1p4vaj5_di" bpmnElement="SendTask">
        <dc:Bounds x="400" y="110" width="100" height="80" />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Event_16hbdyu_di" bpmnElement="endP1">
        <dc:Bounds x="562" y="132" width="36" height="36" />
        <bpmndi:BPMNLabel>
          <dc:Bounds x="564" y="113" width="32" height="14" />
        </bpmndi:BPMNLabel>
      </bpmndi:BPMNShape>
      <bpmndi:BPMNEdge id="Flow_04pas1n_di" bpmnElement="Flow_04pas1n">
        <di:waypoint x="246" y="150" />
        <di:waypoint x="302" y="150" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_1j9q5v2_di" bpmnElement="Flow_1j9q5v2">
        <di:waypoint x="338" y="150" />
        <di:waypoint x="400" y="150" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_11etsb2_di" bpmnElement="Flow_11etsb2">
        <di:waypoint x="500" y="150" />
        <di:waypoint x="562" y="150" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNShape id="Participant_0u469ig_di" bpmnElement="p2" isHorizontal="true">
        <dc:Bounds x="160" y="290" width="520" height="120" />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Event_0t2uf38_di" bpmnElement="startP2">
        <dc:Bounds x="212" y="332" width="36" height="36" />
        <bpmndi:BPMNLabel>
          <dc:Bounds x="213" y="375" width="35" height="14" />
        </bpmndi:BPMNLabel>
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Event_101v680_di" bpmnElement="receiveEvent">
        <dc:Bounds x="322" y="332" width="36" height="36" />
        <bpmndi:BPMNLabel>
          <dc:Bounds x="308" y="375" width="65" height="14" />
        </bpmndi:BPMNLabel>
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Activity_05hfezq_di" bpmnElement="ReceiveTask">
        <dc:Bounds x="440" y="310" width="100" height="80" />
        <bpmndi:BPMNLabel />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Event_0182ilz_di" bpmnElement="endP2">
        <dc:Bounds x="622" y="332" width="36" height="36" />
        <bpmndi:BPMNLabel>
          <dc:Bounds x="624" y="308" width="32" height="14" />
        </bpmndi:BPMNLabel>
      </bpmndi:BPMNShape>
      <bpmndi:BPMNEdge id="Flow_1osubad_di" bpmnElement="Flow_1osubad">
        <di:waypoint x="248" y="350" />
        <di:waypoint x="322" y="350" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_0vrh0u6_di" bpmnElement="Flow_0vrh0u6">
        <di:waypoint x="358" y="350" />
        <di:waypoint x="440" y="350" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_1dm46gf_di" bpmnElement="Flow_1dm46gf">
        <di:waypoint x="540" y="350" />
        <di:waypoint x="622" y="350" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNShape id="BPMNShape_08vdfj5" bpmnElement="p3" isHorizontal="true">
        <dc:Bounds x="160" y="450" width="720" height="270" />
        <bpmndi:BPMNLabel />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Event_014j9ca_di" bpmnElement="Event_0kdhzc1">
        <dc:Bounds x="212" y="492" width="36" height="36" />
        <bpmndi:BPMNLabel>
          <dc:Bounds x="213" y="535" width="35" height="14" />
        </bpmndi:BPMNLabel>
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Gateway_1hzxmwf_di" bpmnElement="Gateway_1pgg1e7">
        <dc:Bounds x="305" y="485" width="50" height="50" />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Event_0wpufyp_di" bpmnElement="Event_0wpufyp">
        <dc:Bounds x="412" y="492" width="36" height="36" />
        <bpmndi:BPMNLabel>
          <dc:Bounds x="418" y="535" width="24" height="14" />
        </bpmndi:BPMNLabel>
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Event_1ek995l_di" bpmnElement="Event_1ek995l">
        <dc:Bounds x="562" y="492" width="36" height="36" />
        <bpmndi:BPMNLabel>
          <dc:Bounds x="564" y="473" width="31" height="14" />
        </bpmndi:BPMNLabel>
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Event_0acx3tz_di" bpmnElement="Event_0acx3tz">
        <dc:Bounds x="412" y="602" width="36" height="36" />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Gateway_1gibfd6_di" bpmnElement="Gateway_1gibfd6" isMarkerVisible="true">
        <dc:Bounds x="615" y="595" width="50" height="50" />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Event_0w1ysb8_di" bpmnElement="Event_0w1ysb8">
        <dc:Bounds x="732" y="602" width="36" height="36" />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Gateway_19osqg6_di" bpmnElement="Gateway_0ff8w9b">
        <dc:Bounds x="505" y="595" width="50" height="50" />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNEdge id="Flow_1cnujy1_di" bpmnElement="Flow_1cnujy1">
        <di:waypoint x="248" y="510" />
        <di:waypoint x="305" y="510" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_06n5qhb_di" bpmnElement="Flow_06n5qhb">
        <di:waypoint x="355" y="510" />
        <di:waypoint x="412" y="510" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_1i5w9wa_di" bpmnElement="Flow_1i5w9wa">
        <di:waypoint x="448" y="510" />
        <di:waypoint x="562" y="510" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_1qh0vc5_di" bpmnElement="Flow_1qh0vc5">
        <di:waypoint x="330" y="535" />
        <di:waypoint x="330" y="620" />
        <di:waypoint x="412" y="620" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_1qw78et_di" bpmnElement="Flow_1qw78et">
        <di:waypoint x="448" y="620" />
        <di:waypoint x="505" y="620" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_03hf6ha_di" bpmnElement="Flow_03hf6ha">
        <di:waypoint x="555" y="620" />
        <di:waypoint x="615" y="620" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_1b3pv18_di" bpmnElement="Flow_1b3pv18">
        <di:waypoint x="665" y="620" />
        <di:waypoint x="732" y="620" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_0huzdij_di" bpmnElement="Flow_0huzdij">
        <di:waypoint x="530" y="645" />
        <di:waypoint x="530" y="680" />
        <di:waypoint x="640" y="680" />
        <di:waypoint x="640" y="645" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_0mj6zqd_di" bpmnElement="Flow_0mj6zqd">
        <di:waypoint x="320" y="168" />
        <di:waypoint x="320" y="240" />
        <di:waypoint x="230" y="240" />
        <di:waypoint x="230" y="332" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_1l5tj5s_di" bpmnElement="Flow_1l5tj5s">
        <di:waypoint x="450" y="190" />
        <di:waypoint x="450" y="236" />
        <di:waypoint x="340" y="236" />
        <di:waypoint x="340" y="332" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_1teo7b1_di" bpmnElement="Flow_1teo7b1">
        <di:waypoint x="580" y="168" />
        <di:waypoint x="580" y="239" />
        <di:waypoint x="490" y="239" />
        <di:waypoint x="490" y="310" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_1k7qfrf_di" bpmnElement="Flow_1k7qfrf">
        <di:waypoint x="640" y="368" />
        <di:waypoint x="640" y="430" />
        <di:waypoint x="430" y="430" />
        <di:waypoint x="430" y="492" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_1ukddvo_di" bpmnElement="Flow_1ukddvo">
        <di:waypoint x="640" y="368" />
        <di:waypoint x="640" y="560" />
        <di:waypoint x="430" y="560" />
        <di:waypoint x="430" y="602" />
      </bpmndi:BPMNEdge>
    </bpmndi:BPMNPlane>
  </bpmndi:BPMNDiagram>
</bpmn:definitions>
`,cycles:`<?xml version="1.0" encoding="UTF-8"?>
<definitions xmlns:bpmndi="http://www.omg.org/spec/BPMN/20100524/DI" xmlns:dc="http://www.omg.org/spec/DD/20100524/DC" xmlns:di="http://www.omg.org/spec/DD/20100524/DI" xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL" id="definitions_793d1020-e08a-4f53-b128-c61827f033c9" targetNamespace="http://www.omg.org/spec/BPMN/20100524/MODEL" exporter="Camunda Modeler" exporterVersion="5.26.0">
  <process id="process_c648aa44-f99b-4cc0-8bba-9b1fafd7d01b" isExecutable="false">
    <startEvent id="FlowNode_1" name="start">
      <outgoing>Flow_18hry6m</outgoing>
    </startEvent>
    <sequenceFlow id="Flow_18hry6m" sourceRef="FlowNode_1" targetRef="Gateway_187h8sm" />
    <task id="Activity_1k9hlq7" name="A">
      <incoming>Flow_0k243p1</incoming>
      <outgoing>Flow_0msbshj</outgoing>
    </task>
    <sequenceFlow id="Flow_0k243p1" sourceRef="Gateway_19m1xkh" targetRef="Activity_1k9hlq7" />
    <sequenceFlow id="Flow_0msbshj" sourceRef="Activity_1k9hlq7" targetRef="Gateway_17yykq8" />
    <exclusiveGateway id="Gateway_17yykq8" name="e1">
      <incoming>Flow_0msbshj</incoming>
      <outgoing>Flow_0yhwwde</outgoing>
      <outgoing>Flow_0flugpi</outgoing>
    </exclusiveGateway>
    <endEvent id="Event_0er1rer" name="end">
      <incoming>Flow_1jm84qb</incoming>
    </endEvent>
    <sequenceFlow id="Flow_0yhwwde" sourceRef="Gateway_17yykq8" targetRef="Gateway_046w909" />
    <sequenceFlow id="Flow_0flugpi" sourceRef="Gateway_17yykq8" targetRef="Gateway_19m1xkh" />
    <exclusiveGateway id="Gateway_19m1xkh" name="p1">
      <incoming>Flow_0flugpi</incoming>
      <incoming>Flow_1qld4t5</incoming>
      <outgoing>Flow_0k243p1</outgoing>
    </exclusiveGateway>
    <sequenceFlow id="Flow_1qld4t5" sourceRef="Gateway_187h8sm" targetRef="Gateway_19m1xkh" />
    <parallelGateway id="Gateway_187h8sm">
      <incoming>Flow_18hry6m</incoming>
      <outgoing>Flow_1qld4t5</outgoing>
      <outgoing>Flow_18zc06g</outgoing>
    </parallelGateway>
    <sequenceFlow id="Flow_1jm84qb" sourceRef="Gateway_046w909" targetRef="Event_0er1rer" />
    <sequenceFlow id="Flow_18zc06g" sourceRef="Gateway_187h8sm" targetRef="Gateway_046w909" />
    <exclusiveGateway id="Gateway_046w909">
      <incoming>Flow_0yhwwde</incoming>
      <incoming>Flow_18zc06g</incoming>
      <outgoing>Flow_1jm84qb</outgoing>
    </exclusiveGateway>
    <startEvent id="Event_1xene0c" />
    <textAnnotation id="TextAnnotation_0y9eh5z">
      <text>Tests if a quick fix can be found by following the incoming flows which contain a cycle.</text>
    </textAnnotation>
    <association id="Association_1y0c19p" associationDirection="None" sourceRef="Gateway_046w909" targetRef="TextAnnotation_0y9eh5z" />
  </process>
  <bpmndi:BPMNDiagram id="BPMNDiagram_40e0d24a-67a5-413e-aefc-dad265aaf73b">
    <bpmndi:BPMNPlane id="BPMNPlane_500f5dcb-d27a-4260-8cac-962e6eb35e01" bpmnElement="process_c648aa44-f99b-4cc0-8bba-9b1fafd7d01b">
      <bpmndi:BPMNShape id="BPMNShape_5604032a-8541-4e0f-9276-10d9de3c10df" bpmnElement="FlowNode_1">
        <dc:Bounds x="152" y="192" width="36" height="36" />
        <bpmndi:BPMNLabel>
          <dc:Bounds x="159" y="235" width="23" height="14" />
        </bpmndi:BPMNLabel>
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Gateway_17yykq8_di" bpmnElement="Gateway_17yykq8" isMarkerVisible="true">
        <dc:Bounds x="585" y="185" width="50" height="50" />
        <bpmndi:BPMNLabel>
          <dc:Bounds x="604" y="245" width="13" height="14" />
        </bpmndi:BPMNLabel>
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Event_0er1rer_di" bpmnElement="Event_0er1rer">
        <dc:Bounds x="782" y="192" width="36" height="36" />
        <bpmndi:BPMNLabel>
          <dc:Bounds x="791" y="235" width="19" height="14" />
        </bpmndi:BPMNLabel>
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Gateway_0jrtwhx_di" bpmnElement="Gateway_19m1xkh" isMarkerVisible="true">
        <dc:Bounds x="305" y="185" width="50" height="50" />
        <bpmndi:BPMNLabel>
          <dc:Bounds x="323" y="245" width="13" height="14" />
        </bpmndi:BPMNLabel>
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Gateway_1foymp0_di" bpmnElement="Gateway_187h8sm">
        <dc:Bounds x="225" y="185" width="50" height="50" />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Gateway_0ofdfbx_di" bpmnElement="Gateway_046w909" isMarkerVisible="true">
        <dc:Bounds x="685" y="185" width="50" height="50" />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Event_1xene0c_di" bpmnElement="Event_1xene0c">
        <dc:Bounds x="252" y="332" width="36" height="36" />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Activity_1k9hlq7_di" bpmnElement="Activity_1k9hlq7">
        <dc:Bounds x="430" y="170" width="100" height="80" />
        <bpmndi:BPMNLabel />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="TextAnnotation_0y9eh5z_di" bpmnElement="TextAnnotation_0y9eh5z">
        <dc:Bounds x="750" y="81" width="100" height="98" />
        <bpmndi:BPMNLabel />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNEdge id="Flow_18hry6m_di" bpmnElement="Flow_18hry6m">
        <di:waypoint x="188" y="210" />
        <di:waypoint x="225" y="210" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_0k243p1_di" bpmnElement="Flow_0k243p1">
        <di:waypoint x="355" y="210" />
        <di:waypoint x="430" y="210" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_0msbshj_di" bpmnElement="Flow_0msbshj">
        <di:waypoint x="530" y="210" />
        <di:waypoint x="585" y="210" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_0yhwwde_di" bpmnElement="Flow_0yhwwde">
        <di:waypoint x="635" y="210" />
        <di:waypoint x="685" y="210" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_0flugpi_di" bpmnElement="Flow_0flugpi">
        <di:waypoint x="610" y="185" />
        <di:waypoint x="610" y="130" />
        <di:waypoint x="330" y="130" />
        <di:waypoint x="330" y="185" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_1qld4t5_di" bpmnElement="Flow_1qld4t5">
        <di:waypoint x="275" y="210" />
        <di:waypoint x="305" y="210" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_1jm84qb_di" bpmnElement="Flow_1jm84qb">
        <di:waypoint x="735" y="210" />
        <di:waypoint x="782" y="210" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_18zc06g_di" bpmnElement="Flow_18zc06g">
        <di:waypoint x="250" y="235" />
        <di:waypoint x="250" y="290" />
        <di:waypoint x="710" y="290" />
        <di:waypoint x="710" y="235" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Association_1y0c19p_di" bpmnElement="Association_1y0c19p">
        <di:waypoint x="721" y="196" />
        <di:waypoint x="750" y="159" />
      </bpmndi:BPMNEdge>
    </bpmndi:BPMNPlane>
  </bpmndi:BPMNDiagram>
</definitions>
`,deadReceiveTask:`<?xml version="1.0" encoding="UTF-8"?>
<definitions xmlns:bpmndi="http://www.omg.org/spec/BPMN/20100524/DI" xmlns:dc="http://www.omg.org/spec/DD/20100524/DC" xmlns:di="http://www.omg.org/spec/DD/20100524/DI" xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL" id="definitions_793d1020-e08a-4f53-b128-c61827f033c9" targetNamespace="http://www.omg.org/spec/BPMN/20100524/MODEL" exporter="Camunda Modeler" exporterVersion="5.20.0">
  <collaboration id="Collaboration_0k5zawz">
    <participant id="Participant_0dxobkw" name="Process1" processRef="process_c648aa44-f99b-4cc0-8bba-9b1fafd7d01b" />
    <participant id="Participant_11uor0t" name="Process2" processRef="Process_1jpk9vo" />
    <textAnnotation id="TextAnnotation_0x522ef">
      <text>This should not be picked for a new MF as quick fix.</text>
    </textAnnotation>
    <textAnnotation id="TextAnnotation_0ac1r39">
      <text>Cannot be executed due to missing MF.</text>
    </textAnnotation>
    <association id="Association_09in8a1" associationDirection="None" sourceRef="Event_0irq846" targetRef="TextAnnotation_0x522ef" />
    <association id="Association_085mqcj" associationDirection="None" sourceRef="Activity_160vzta" targetRef="TextAnnotation_0ac1r39" />
  </collaboration>
  <process id="process_c648aa44-f99b-4cc0-8bba-9b1fafd7d01b" isExecutable="false">
    <startEvent id="Event_03hdyxt">
      <outgoing>Flow_038tgij</outgoing>
    </startEvent>
    <receiveTask id="Activity_160vzta">
      <incoming>Flow_038tgij</incoming>
      <outgoing>Flow_1dxoah9</outgoing>
    </receiveTask>
    <endEvent id="Event_0irq846">
      <incoming>Flow_1dxoah9</incoming>
      <messageEventDefinition id="MessageEventDefinition_0k3rgk1" />
    </endEvent>
    <sequenceFlow id="Flow_038tgij" sourceRef="Event_03hdyxt" targetRef="Activity_160vzta" />
    <sequenceFlow id="Flow_1dxoah9" sourceRef="Activity_160vzta" targetRef="Event_0irq846" />
  </process>
  <process id="Process_1jpk9vo" isExecutable="false">
    <startEvent id="Event_10w6x6m">
      <outgoing>Flow_1svwxja</outgoing>
    </startEvent>
    <endEvent id="Event_10suyeb">
      <incoming>Flow_1svwxja</incoming>
      <messageEventDefinition id="MessageEventDefinition_1qmha59" />
    </endEvent>
    <sequenceFlow id="Flow_1svwxja" sourceRef="Event_10w6x6m" targetRef="Event_10suyeb" />
  </process>
  <bpmndi:BPMNDiagram id="BPMNDiagram_40e0d24a-67a5-413e-aefc-dad265aaf73b">
    <bpmndi:BPMNPlane id="BPMNPlane_500f5dcb-d27a-4260-8cac-962e6eb35e01" bpmnElement="Collaboration_0k5zawz">
      <bpmndi:BPMNShape id="Participant_0dxobkw_di" bpmnElement="Participant_0dxobkw" isHorizontal="true">
        <dc:Bounds x="150" y="160" width="380" height="160" />
        <bpmndi:BPMNLabel />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Event_03hdyxt_di" bpmnElement="Event_03hdyxt">
        <dc:Bounds x="202" y="222" width="36" height="36" />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Activity_01lhmtn_di" bpmnElement="Activity_160vzta">
        <dc:Bounds x="290" y="200" width="100" height="80" />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Event_1m7j251_di" bpmnElement="Event_0irq846">
        <dc:Bounds x="442" y="222" width="36" height="36" />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNEdge id="Flow_038tgij_di" bpmnElement="Flow_038tgij">
        <di:waypoint x="238" y="240" />
        <di:waypoint x="290" y="240" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_1dxoah9_di" bpmnElement="Flow_1dxoah9">
        <di:waypoint x="390" y="240" />
        <di:waypoint x="442" y="240" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNShape id="BPMNShape_1ytn0zr" bpmnElement="Participant_11uor0t" isHorizontal="true">
        <dc:Bounds x="150" y="347" width="380" height="160" />
        <bpmndi:BPMNLabel />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="BPMNShape_04p9mz1" bpmnElement="Event_10w6x6m">
        <dc:Bounds x="202" y="409" width="36" height="36" />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Event_09g6nyb_di" bpmnElement="Event_10suyeb">
        <dc:Bounds x="442" y="409" width="36" height="36" />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNEdge id="Flow_1svwxja_di" bpmnElement="Flow_1svwxja">
        <di:waypoint x="238" y="427" />
        <di:waypoint x="442" y="427" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNShape id="TextAnnotation_0x522ef_di" bpmnElement="TextAnnotation_0x522ef">
        <dc:Bounds x="560" y="200" width="100" height="70" />
        <bpmndi:BPMNLabel />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="TextAnnotation_0ac1r39_di" bpmnElement="TextAnnotation_0ac1r39">
        <dc:Bounds x="420" y="80" width="100" height="55" />
        <bpmndi:BPMNLabel />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNEdge id="Association_09in8a1_di" bpmnElement="Association_09in8a1">
        <di:waypoint x="478" y="237" />
        <di:waypoint x="560" y="223" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Association_085mqcj_di" bpmnElement="Association_085mqcj">
        <di:waypoint x="378" y="200" />
        <di:waypoint x="440" y="135" />
      </bpmndi:BPMNEdge>
    </bpmndi:BPMNPlane>
  </bpmndi:BPMNDiagram>
</definitions>
`,deadMice:`<?xml version="1.0" encoding="UTF-8"?>
<definitions xmlns:bpmndi="http://www.omg.org/spec/BPMN/20100524/DI" xmlns:dc="http://www.omg.org/spec/DD/20100524/DC" xmlns:di="http://www.omg.org/spec/DD/20100524/DI" xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL" id="definitions_793d1020-e08a-4f53-b128-c61827f033c9" targetNamespace="http://www.omg.org/spec/BPMN/20100524/MODEL" exporter="Camunda Modeler" exporterVersion="5.20.0">
  <collaboration id="Collaboration_0k5zawz">
    <participant id="Participant_0dxobkw" name="Process1" processRef="process_c648aa44-f99b-4cc0-8bba-9b1fafd7d01b" />
    <participant id="Participant_11uor0t" name="Process2" processRef="Process_1jpk9vo" />
    <textAnnotation id="TextAnnotation_0x522ef">
      <text>This should not be picked for a new MF as quick fix.</text>
    </textAnnotation>
    <association id="Association_09in8a1" associationDirection="None" sourceRef="Event_0irq846" targetRef="TextAnnotation_0x522ef" />
    <textAnnotation id="TextAnnotation_03t1sb5">
      <text>Cannot be executed due to missing MF.</text>
    </textAnnotation>
    <association id="Association_0o0ylem" associationDirection="None" sourceRef="Event_1kb3eg2" targetRef="TextAnnotation_03t1sb5" />
  </collaboration>
  <process id="process_c648aa44-f99b-4cc0-8bba-9b1fafd7d01b" isExecutable="false">
    <startEvent id="Event_03hdyxt">
      <outgoing>Flow_038tgij</outgoing>
    </startEvent>
    <sequenceFlow id="Flow_038tgij" sourceRef="Event_03hdyxt" targetRef="Event_1kb3eg2" />
    <endEvent id="Event_0irq846">
      <incoming>Flow_1lfeba5</incoming>
      <messageEventDefinition id="MessageEventDefinition_0k3rgk1" />
    </endEvent>
    <sequenceFlow id="Flow_1lfeba5" sourceRef="Event_1kb3eg2" targetRef="Event_0irq846" />
    <intermediateCatchEvent id="Event_1kb3eg2">
      <incoming>Flow_038tgij</incoming>
      <outgoing>Flow_1lfeba5</outgoing>
      <messageEventDefinition id="MessageEventDefinition_00tp03u" />
    </intermediateCatchEvent>
  </process>
  <process id="Process_1jpk9vo" isExecutable="false">
    <startEvent id="Event_10w6x6m">
      <outgoing>Flow_1ju8swo</outgoing>
    </startEvent>
    <sequenceFlow id="Flow_1ju8swo" sourceRef="Event_10w6x6m" targetRef="Event_10suyeb" />
    <endEvent id="Event_10suyeb">
      <incoming>Flow_1ju8swo</incoming>
      <messageEventDefinition id="MessageEventDefinition_1qmha59" />
    </endEvent>
  </process>
  <bpmndi:BPMNDiagram id="BPMNDiagram_40e0d24a-67a5-413e-aefc-dad265aaf73b">
    <bpmndi:BPMNPlane id="BPMNPlane_500f5dcb-d27a-4260-8cac-962e6eb35e01" bpmnElement="Collaboration_0k5zawz">
      <bpmndi:BPMNShape id="Participant_0dxobkw_di" bpmnElement="Participant_0dxobkw" isHorizontal="true">
        <dc:Bounds x="150" y="110" width="380" height="160" />
        <bpmndi:BPMNLabel />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Event_03hdyxt_di" bpmnElement="Event_03hdyxt">
        <dc:Bounds x="202" y="172" width="36" height="36" />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Event_1m7j251_di" bpmnElement="Event_0irq846">
        <dc:Bounds x="442" y="172" width="36" height="36" />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Event_16serj1_di" bpmnElement="Event_1kb3eg2">
        <dc:Bounds x="322" y="172" width="36" height="36" />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNEdge id="Flow_038tgij_di" bpmnElement="Flow_038tgij">
        <di:waypoint x="238" y="190" />
        <di:waypoint x="322" y="190" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_1lfeba5_di" bpmnElement="Flow_1lfeba5">
        <di:waypoint x="358" y="190" />
        <di:waypoint x="442" y="190" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNShape id="BPMNShape_1ytn0zr" bpmnElement="Participant_11uor0t" isHorizontal="true">
        <dc:Bounds x="150" y="299" width="380" height="160" />
        <bpmndi:BPMNLabel />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="BPMNShape_04p9mz1" bpmnElement="Event_10w6x6m">
        <dc:Bounds x="202" y="361" width="36" height="36" />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Event_09g6nyb_di" bpmnElement="Event_10suyeb">
        <dc:Bounds x="322" y="361" width="36" height="36" />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNEdge id="Flow_1ju8swo_di" bpmnElement="Flow_1ju8swo">
        <di:waypoint x="238" y="379" />
        <di:waypoint x="322" y="379" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNShape id="TextAnnotation_0x522ef_di" bpmnElement="TextAnnotation_0x522ef">
        <dc:Bounds x="560" y="150" width="100" height="70" />
        <bpmndi:BPMNLabel />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="TextAnnotation_03t1sb5_di" bpmnElement="TextAnnotation_03t1sb5">
        <dc:Bounds x="440" y="40" width="100" height="55" />
        <bpmndi:BPMNLabel />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNEdge id="Association_09in8a1_di" bpmnElement="Association_09in8a1">
        <di:waypoint x="478" y="187" />
        <di:waypoint x="560" y="173" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Association_0o0ylem_di" bpmnElement="Association_0o0ylem">
        <di:waypoint x="354" y="178" />
        <di:waypoint x="452" y="95" />
      </bpmndi:BPMNEdge>
    </bpmndi:BPMNPlane>
  </bpmndi:BPMNDiagram>
</definitions>
`,starvation:`<?xml version="1.0" encoding="UTF-8"?>
<definitions xmlns:bpmndi="http://www.omg.org/spec/BPMN/20100524/DI" xmlns:dc="http://www.omg.org/spec/DD/20100524/DC" xmlns:di="http://www.omg.org/spec/DD/20100524/DI" xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL" id="definitions_793d1020-e08a-4f53-b128-c61827f033c9" targetNamespace="http://www.omg.org/spec/BPMN/20100524/MODEL" exporter="Camunda Modeler" exporterVersion="5.19.0">
  <collaboration id="Collaboration_0vol473">
    <participant id="Participant_13gi6fo" name="1" processRef="process_c648aa44-f99b-4cc0-8bba-9b1fafd7d01b" />
    <participant id="Participant_05lnuh3" name="2" processRef="Process_0y0s9t9" />
    <messageFlow id="Flow_0x1mpx3" sourceRef="Activity_0evks5b" targetRef="Activity_1bp5b5q" />
  </collaboration>
  <process id="process_c648aa44-f99b-4cc0-8bba-9b1fafd7d01b" isExecutable="false">
    <startEvent id="Event_0qfwbor">
      <outgoing>Flow_0c7a4p3</outgoing>
    </startEvent>
    <sequenceFlow id="Flow_0c7a4p3" sourceRef="Event_0qfwbor" targetRef="Gateway_0z322t8" />
    <parallelGateway id="Gateway_0z322t8">
      <incoming>Flow_0c7a4p3</incoming>
      <outgoing>Flow_12o7cu8</outgoing>
      <outgoing>Flow_0k74cvz</outgoing>
    </parallelGateway>
    <task id="Activity_0725u4y" name="A">
      <incoming>Flow_12o7cu8</incoming>
      <outgoing>Flow_1ob1z5r</outgoing>
    </task>
    <sequenceFlow id="Flow_12o7cu8" sourceRef="Gateway_0z322t8" targetRef="Activity_0725u4y" />
    <sequenceFlow id="Flow_0k74cvz" sourceRef="Gateway_0z322t8" targetRef="Activity_1bp5b5q" />
    <sequenceFlow id="Flow_1ob1z5r" sourceRef="Activity_0725u4y" targetRef="Gateway_0hk51u8" />
    <parallelGateway id="Gateway_0hk51u8">
      <incoming>Flow_1ob1z5r</incoming>
      <incoming>Flow_0b19npu</incoming>
      <outgoing>Flow_1hnlh03</outgoing>
    </parallelGateway>
    <sequenceFlow id="Flow_0b19npu" sourceRef="Activity_1bp5b5q" targetRef="Gateway_0hk51u8" />
    <task id="Activity_0ybihf9" name="C">
      <incoming>Flow_1hnlh03</incoming>
      <outgoing>Flow_0ah32ra</outgoing>
    </task>
    <sequenceFlow id="Flow_1hnlh03" sourceRef="Gateway_0hk51u8" targetRef="Activity_0ybihf9" />
    <endEvent id="Event_13ygpus">
      <incoming>Flow_0ah32ra</incoming>
    </endEvent>
    <sequenceFlow id="Flow_0ah32ra" sourceRef="Activity_0ybihf9" targetRef="Event_13ygpus" />
    <receiveTask id="Activity_1bp5b5q" name="B">
      <incoming>Flow_0k74cvz</incoming>
      <outgoing>Flow_0b19npu</outgoing>
    </receiveTask>
  </process>
  <process id="Process_0y0s9t9">
    <startEvent id="Event_098c493">
      <outgoing>Flow_1w2t0y5</outgoing>
    </startEvent>
    <exclusiveGateway id="Gateway_004o3nd">
      <incoming>Flow_1w2t0y5</incoming>
      <outgoing>Flow_0azh6sy</outgoing>
      <outgoing>Flow_0kor54a</outgoing>
    </exclusiveGateway>
    <sequenceFlow id="Flow_1w2t0y5" sourceRef="Event_098c493" targetRef="Gateway_004o3nd" />
    <sequenceFlow id="Flow_0azh6sy" sourceRef="Gateway_004o3nd" targetRef="Activity_0evks5b" />
    <task id="Activity_1rcssnk" name="B2">
      <incoming>Flow_0kor54a</incoming>
      <outgoing>Flow_0qyt4da</outgoing>
    </task>
    <sequenceFlow id="Flow_0kor54a" sourceRef="Gateway_004o3nd" targetRef="Activity_1rcssnk" />
    <exclusiveGateway id="Gateway_1nbmve5">
      <incoming>Flow_0vxuckn</incoming>
      <incoming>Flow_0qyt4da</incoming>
      <outgoing>Flow_0el6hk1</outgoing>
    </exclusiveGateway>
    <sequenceFlow id="Flow_0vxuckn" sourceRef="Activity_0evks5b" targetRef="Gateway_1nbmve5" />
    <sequenceFlow id="Flow_0qyt4da" sourceRef="Activity_1rcssnk" targetRef="Gateway_1nbmve5" />
    <task id="Activity_09p2aui">
      <incoming>Flow_0el6hk1</incoming>
      <outgoing>Flow_1qeo6nl</outgoing>
    </task>
    <sequenceFlow id="Flow_0el6hk1" sourceRef="Gateway_1nbmve5" targetRef="Activity_09p2aui" />
    <endEvent id="Event_11ylisx">
      <incoming>Flow_1qeo6nl</incoming>
    </endEvent>
    <sequenceFlow id="Flow_1qeo6nl" sourceRef="Activity_09p2aui" targetRef="Event_11ylisx" />
    <sendTask id="Activity_0evks5b" name="A2">
      <incoming>Flow_0azh6sy</incoming>
      <outgoing>Flow_0vxuckn</outgoing>
    </sendTask>
  </process>
  <bpmndi:BPMNDiagram id="BPMNDiagram_40e0d24a-67a5-413e-aefc-dad265aaf73b">
    <bpmndi:BPMNPlane id="BPMNPlane_500f5dcb-d27a-4260-8cac-962e6eb35e01" bpmnElement="Collaboration_0vol473">
      <bpmndi:BPMNShape id="Participant_13gi6fo_di" bpmnElement="Participant_13gi6fo" isHorizontal="true">
        <dc:Bounds x="170" y="190" width="858" height="320" />
        <bpmndi:BPMNLabel />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Event_0qfwbor_di" bpmnElement="Event_0qfwbor">
        <dc:Bounds x="232" y="282" width="36" height="36" />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Gateway_075sma8_di" bpmnElement="Gateway_0z322t8">
        <dc:Bounds x="325" y="275" width="50" height="50" />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Activity_0725u4y_di" bpmnElement="Activity_0725u4y">
        <dc:Bounds x="440" y="260" width="100" height="80" />
        <bpmndi:BPMNLabel />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Gateway_1ycpifz_di" bpmnElement="Gateway_0hk51u8">
        <dc:Bounds x="605" y="275" width="50" height="50" />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Activity_0ybihf9_di" bpmnElement="Activity_0ybihf9">
        <dc:Bounds x="720" y="260" width="100" height="80" />
        <bpmndi:BPMNLabel />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Event_13ygpus_di" bpmnElement="Event_13ygpus">
        <dc:Bounds x="892" y="282" width="36" height="36" />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Activity_1ui12op_di" bpmnElement="Activity_1bp5b5q">
        <dc:Bounds x="440" y="370" width="100" height="80" />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNEdge id="Flow_0c7a4p3_di" bpmnElement="Flow_0c7a4p3">
        <di:waypoint x="268" y="300" />
        <di:waypoint x="325" y="300" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_12o7cu8_di" bpmnElement="Flow_12o7cu8">
        <di:waypoint x="375" y="300" />
        <di:waypoint x="440" y="300" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_0k74cvz_di" bpmnElement="Flow_0k74cvz">
        <di:waypoint x="350" y="325" />
        <di:waypoint x="350" y="410" />
        <di:waypoint x="440" y="410" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_1ob1z5r_di" bpmnElement="Flow_1ob1z5r">
        <di:waypoint x="540" y="300" />
        <di:waypoint x="605" y="300" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_0b19npu_di" bpmnElement="Flow_0b19npu">
        <di:waypoint x="540" y="410" />
        <di:waypoint x="630" y="410" />
        <di:waypoint x="630" y="325" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_1hnlh03_di" bpmnElement="Flow_1hnlh03">
        <di:waypoint x="655" y="300" />
        <di:waypoint x="720" y="300" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_0ah32ra_di" bpmnElement="Flow_0ah32ra">
        <di:waypoint x="820" y="300" />
        <di:waypoint x="892" y="300" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNShape id="Participant_05lnuh3_di" bpmnElement="Participant_05lnuh3" isHorizontal="true">
        <dc:Bounds x="170" y="560" width="858" height="340" />
        <bpmndi:BPMNLabel />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Event_098c493_di" bpmnElement="Event_098c493">
        <dc:Bounds x="232" y="672" width="36" height="36" />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Gateway_004o3nd_di" bpmnElement="Gateway_004o3nd" isMarkerVisible="true">
        <dc:Bounds x="325" y="665" width="50" height="50" />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Activity_1rcssnk_di" bpmnElement="Activity_1rcssnk">
        <dc:Bounds x="440" y="760" width="100" height="80" />
        <bpmndi:BPMNLabel />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Gateway_1nbmve5_di" bpmnElement="Gateway_1nbmve5" isMarkerVisible="true">
        <dc:Bounds x="605" y="665" width="50" height="50" />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Activity_09p2aui_di" bpmnElement="Activity_09p2aui">
        <dc:Bounds x="720" y="650" width="100" height="80" />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Event_11ylisx_di" bpmnElement="Event_11ylisx">
        <dc:Bounds x="892" y="672" width="36" height="36" />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Activity_034246t_di" bpmnElement="Activity_0evks5b">
        <dc:Bounds x="440" y="650" width="100" height="80" />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNEdge id="Flow_1w2t0y5_di" bpmnElement="Flow_1w2t0y5">
        <di:waypoint x="268" y="690" />
        <di:waypoint x="325" y="690" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_0azh6sy_di" bpmnElement="Flow_0azh6sy">
        <di:waypoint x="375" y="690" />
        <di:waypoint x="440" y="690" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_0kor54a_di" bpmnElement="Flow_0kor54a">
        <di:waypoint x="350" y="715" />
        <di:waypoint x="350" y="800" />
        <di:waypoint x="440" y="800" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_0vxuckn_di" bpmnElement="Flow_0vxuckn">
        <di:waypoint x="540" y="690" />
        <di:waypoint x="605" y="690" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_0qyt4da_di" bpmnElement="Flow_0qyt4da">
        <di:waypoint x="540" y="800" />
        <di:waypoint x="630" y="800" />
        <di:waypoint x="630" y="715" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_0el6hk1_di" bpmnElement="Flow_0el6hk1">
        <di:waypoint x="655" y="690" />
        <di:waypoint x="720" y="690" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_1qeo6nl_di" bpmnElement="Flow_1qeo6nl">
        <di:waypoint x="820" y="690" />
        <di:waypoint x="892" y="690" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_0x1mpx3_di" bpmnElement="Flow_0x1mpx3">
        <di:waypoint x="490" y="650" />
        <di:waypoint x="490" y="450" />
      </bpmndi:BPMNEdge>
    </bpmndi:BPMNPlane>
  </bpmndi:BPMNDiagram>
</definitions>
`,livelock:`<?xml version="1.0" encoding="UTF-8"?>
<definitions xmlns:bpmndi="http://www.omg.org/spec/BPMN/20100524/DI" xmlns:dc="http://www.omg.org/spec/DD/20100524/DC" xmlns:di="http://www.omg.org/spec/DD/20100524/DI" xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL" id="definitions_793d1020-e08a-4f53-b128-c61827f033c9" targetNamespace="http://www.omg.org/spec/BPMN/20100524/MODEL" exporter="Camunda Modeler" exporterVersion="5.19.0">
  <process id="process_c648aa44-f99b-4cc0-8bba-9b1fafd7d01b" isExecutable="false">
    <startEvent id="FlowNode_1" name="start">
      <outgoing>Flow_18hry6m</outgoing>
    </startEvent>
    <sequenceFlow id="Flow_18hry6m" sourceRef="FlowNode_1" targetRef="Gateway_19m1xkh" />
    <exclusiveGateway id="Gateway_19m1xkh" name="p1">
      <incoming>Flow_18hry6m</incoming>
      <incoming>Flow_0m5zopo</incoming>
      <outgoing>Flow_1kuz64n</outgoing>
    </exclusiveGateway>
    <task id="Activity_0hrxbg4" name="B">
      <incoming>Flow_1kuz64n</incoming>
      <outgoing>Flow_1vln6g3</outgoing>
    </task>
    <sequenceFlow id="Flow_1kuz64n" sourceRef="Gateway_19m1xkh" targetRef="Activity_0hrxbg4" />
    <sequenceFlow id="Flow_1vln6g3" sourceRef="Activity_0hrxbg4" targetRef="Gateway_11yg34z" />
    <parallelGateway id="Gateway_11yg34z">
      <incoming>Flow_1vln6g3</incoming>
      <outgoing>Flow_188u7dc</outgoing>
      <outgoing>Flow_102b6is</outgoing>
    </parallelGateway>
    <task id="Activity_1mq2eql" name="A">
      <incoming>Flow_188u7dc</incoming>
      <outgoing>Flow_0m5zopo</outgoing>
    </task>
    <sequenceFlow id="Flow_188u7dc" sourceRef="Gateway_11yg34z" targetRef="Activity_1mq2eql" />
    <task id="Activity_08howcn" name="C">
      <incoming>Flow_102b6is</incoming>
      <outgoing>Flow_1q4upp8</outgoing>
    </task>
    <sequenceFlow id="Flow_102b6is" sourceRef="Gateway_11yg34z" targetRef="Activity_08howcn" />
    <endEvent id="Event_1rkhsm3">
      <incoming>Flow_1q4upp8</incoming>
    </endEvent>
    <sequenceFlow id="Flow_1q4upp8" sourceRef="Activity_08howcn" targetRef="Event_1rkhsm3" />
    <sequenceFlow id="Flow_0m5zopo" sourceRef="Activity_1mq2eql" targetRef="Gateway_19m1xkh" />
  </process>
  <bpmndi:BPMNDiagram id="BPMNDiagram_40e0d24a-67a5-413e-aefc-dad265aaf73b">
    <bpmndi:BPMNPlane id="BPMNPlane_500f5dcb-d27a-4260-8cac-962e6eb35e01" bpmnElement="process_c648aa44-f99b-4cc0-8bba-9b1fafd7d01b">
      <bpmndi:BPMNShape id="BPMNShape_5604032a-8541-4e0f-9276-10d9de3c10df" bpmnElement="FlowNode_1">
        <dc:Bounds x="182" y="92" width="36" height="36" />
        <bpmndi:BPMNLabel>
          <dc:Bounds x="189" y="135" width="23" height="14" />
        </bpmndi:BPMNLabel>
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Gateway_0bxi4eh_di" bpmnElement="Gateway_19m1xkh" isMarkerVisible="true">
        <dc:Bounds x="275" y="85" width="50" height="50" />
        <bpmndi:BPMNLabel>
          <dc:Bounds x="293" y="145" width="14" height="14" />
        </bpmndi:BPMNLabel>
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Activity_0hrxbg4_di" bpmnElement="Activity_0hrxbg4">
        <dc:Bounds x="390" y="70" width="100" height="80" />
        <bpmndi:BPMNLabel />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Gateway_0y14qon_di" bpmnElement="Gateway_11yg34z">
        <dc:Bounds x="555" y="85" width="50" height="50" />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Activity_1mq2eql_di" bpmnElement="Activity_1mq2eql">
        <dc:Bounds x="390" y="-60" width="100" height="80" />
        <bpmndi:BPMNLabel />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Activity_08howcn_di" bpmnElement="Activity_08howcn">
        <dc:Bounds x="670" y="70" width="100" height="80" />
        <bpmndi:BPMNLabel />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Event_1rkhsm3_di" bpmnElement="Event_1rkhsm3">
        <dc:Bounds x="842" y="92" width="36" height="36" />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNEdge id="Flow_18hry6m_di" bpmnElement="Flow_18hry6m">
        <di:waypoint x="218" y="110" />
        <di:waypoint x="275" y="110" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_1kuz64n_di" bpmnElement="Flow_1kuz64n">
        <di:waypoint x="325" y="110" />
        <di:waypoint x="390" y="110" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_1vln6g3_di" bpmnElement="Flow_1vln6g3">
        <di:waypoint x="490" y="110" />
        <di:waypoint x="555" y="110" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_188u7dc_di" bpmnElement="Flow_188u7dc">
        <di:waypoint x="580" y="85" />
        <di:waypoint x="580" y="-20" />
        <di:waypoint x="490" y="-20" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_102b6is_di" bpmnElement="Flow_102b6is">
        <di:waypoint x="605" y="110" />
        <di:waypoint x="670" y="110" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_1q4upp8_di" bpmnElement="Flow_1q4upp8">
        <di:waypoint x="770" y="110" />
        <di:waypoint x="842" y="110" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_0m5zopo_di" bpmnElement="Flow_0m5zopo">
        <di:waypoint x="390" y="-20" />
        <di:waypoint x="300" y="-20" />
        <di:waypoint x="300" y="85" />
      </bpmndi:BPMNEdge>
    </bpmndi:BPMNPlane>
  </bpmndi:BPMNDiagram>
</definitions>
`,deadTasksConnected:`<?xml version="1.0" encoding="UTF-8"?>
<definitions xmlns:bpmndi="http://www.omg.org/spec/BPMN/20100524/DI" xmlns:dc="http://www.omg.org/spec/DD/20100524/DC" xmlns:di="http://www.omg.org/spec/DD/20100524/DI" xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL" id="definitions_793d1020-e08a-4f53-b128-c61827f033c9" targetNamespace="http://www.omg.org/spec/BPMN/20100524/MODEL" exporter="Camunda Modeler" exporterVersion="5.19.0">
  <process id="process_c648aa44-f99b-4cc0-8bba-9b1fafd7d01b" isExecutable="false">
    <startEvent id="FlowNode_1" name="start">
      <outgoing>Flow_0ckbmbp</outgoing>
    </startEvent>
    <task id="Activity_0ke9inj" name="A">
      <incoming>Flow_0ckbmbp</incoming>
      <outgoing>Flow_1w30amw</outgoing>
    </task>
    <sequenceFlow id="Flow_0ckbmbp" sourceRef="FlowNode_1" targetRef="Activity_0ke9inj" />
    <exclusiveGateway id="Gateway_0dcifmn">
      <incoming>Flow_1w30amw</incoming>
      <incoming>Flow_0hzzaf9</incoming>
      <outgoing>Flow_0hey1xg</outgoing>
    </exclusiveGateway>
    <sequenceFlow id="Flow_1w30amw" sourceRef="Activity_0ke9inj" targetRef="Gateway_0dcifmn" />
    <endEvent id="Event_1uushxv">
      <incoming>Flow_0hey1xg</incoming>
    </endEvent>
    <sequenceFlow id="Flow_0hey1xg" sourceRef="Gateway_0dcifmn" targetRef="Event_1uushxv" />
    <exclusiveGateway id="Gateway_0exjytb">
      <incoming>Flow_14p4q5r</incoming>
      <outgoing>Flow_0hzzaf9</outgoing>
      <outgoing>Flow_09nopbi</outgoing>
    </exclusiveGateway>
    <sequenceFlow id="Flow_0hzzaf9" sourceRef="Gateway_0exjytb" targetRef="Gateway_0dcifmn" />
    <task id="Activity_03czb35" name="B">
      <incoming>Flow_1byutdj</incoming>
      <outgoing>Flow_14p4q5r</outgoing>
    </task>
    <task id="Activity_0llc9ag" name="C">
      <incoming>Flow_09nopbi</incoming>
      <outgoing>Flow_1byutdj</outgoing>
    </task>
    <sequenceFlow id="Flow_1byutdj" sourceRef="Activity_0llc9ag" targetRef="Activity_03czb35" />
    <sequenceFlow id="Flow_09nopbi" sourceRef="Gateway_0exjytb" targetRef="Activity_0llc9ag" />
    <sequenceFlow id="Flow_14p4q5r" sourceRef="Activity_03czb35" targetRef="Gateway_0exjytb" />
  </process>
  <bpmndi:BPMNDiagram id="BPMNDiagram_40e0d24a-67a5-413e-aefc-dad265aaf73b">
    <bpmndi:BPMNPlane id="BPMNPlane_500f5dcb-d27a-4260-8cac-962e6eb35e01" bpmnElement="process_c648aa44-f99b-4cc0-8bba-9b1fafd7d01b">
      <bpmndi:BPMNShape id="BPMNShape_5604032a-8541-4e0f-9276-10d9de3c10df" bpmnElement="FlowNode_1">
        <dc:Bounds x="182" y="422" width="36" height="36" />
        <bpmndi:BPMNLabel>
          <dc:Bounds x="189" y="465" width="24" height="14" />
        </bpmndi:BPMNLabel>
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Activity_0ke9inj_di" bpmnElement="Activity_0ke9inj">
        <dc:Bounds x="270" y="400" width="100" height="80" />
        <bpmndi:BPMNLabel />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Gateway_0dcifmn_di" bpmnElement="Gateway_0dcifmn" isMarkerVisible="true">
        <dc:Bounds x="425" y="415" width="50" height="50" />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Event_1uushxv_di" bpmnElement="Event_1uushxv">
        <dc:Bounds x="532" y="422" width="36" height="36" />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Activity_0llc9ag_di" bpmnElement="Activity_0llc9ag">
        <dc:Bounds x="210" y="710" width="100" height="80" />
        <bpmndi:BPMNLabel />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Activity_03czb35_di" bpmnElement="Activity_03czb35">
        <dc:Bounds x="210" y="510" width="100" height="80" />
        <bpmndi:BPMNLabel />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="BPMNShape_0qb1tkz" bpmnElement="Gateway_0exjytb" isMarkerVisible="true">
        <dc:Bounds x="365" y="625" width="50" height="50" />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNEdge id="Flow_0ckbmbp_di" bpmnElement="Flow_0ckbmbp">
        <di:waypoint x="218" y="440" />
        <di:waypoint x="270" y="440" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_1w30amw_di" bpmnElement="Flow_1w30amw">
        <di:waypoint x="370" y="440" />
        <di:waypoint x="425" y="440" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_0hey1xg_di" bpmnElement="Flow_0hey1xg">
        <di:waypoint x="475" y="440" />
        <di:waypoint x="532" y="440" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_0hzzaf9_di" bpmnElement="Flow_0hzzaf9">
        <di:waypoint x="415" y="650" />
        <di:waypoint x="450" y="650" />
        <di:waypoint x="450" y="465" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_1byutdj_di" bpmnElement="Flow_1byutdj">
        <di:waypoint x="210" y="750" />
        <di:waypoint x="170" y="750" />
        <di:waypoint x="170" y="550" />
        <di:waypoint x="210" y="550" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_09nopbi_di" bpmnElement="Flow_09nopbi">
        <di:waypoint x="390" y="675" />
        <di:waypoint x="390" y="750" />
        <di:waypoint x="310" y="750" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_14p4q5r_di" bpmnElement="Flow_14p4q5r">
        <di:waypoint x="310" y="550" />
        <di:waypoint x="390" y="550" />
        <di:waypoint x="390" y="625" />
      </bpmndi:BPMNEdge>
    </bpmndi:BPMNPlane>
  </bpmndi:BPMNDiagram>
</definitions>
`,orderHandling:`<?xml version="1.0" encoding="UTF-8"?>
<definitions xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL" xmlns:bpmndi="http://www.omg.org/spec/BPMN/20100524/DI" xmlns:omgdc="http://www.omg.org/spec/DD/20100524/DC" xmlns:omgdi="http://www.omg.org/spec/DD/20100524/DI" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:bt="http://tk/schema/1.0/bt" targetNamespace="" exporter="Camunda Modeler" exporterVersion="5.20.0" xsi:schemaLocation="http://www.omg.org/spec/BPMN/20100524/MODEL http://www.omg.org/spec/BPMN/2.0/20100501/BPMN20.xsd">
  <collaboration id="Collaboration">
    <extensionElements>
      <bt:processSnapshot id="ProcessSnapshot_0yu1fss" />
    </extensionElements>
    <participant id="Order_handling_p" name="Order handling" processRef="Order_handling" />
    <association id="Association_0y4i2zy" sourceRef="Order_handling_p" />
  </collaboration>
  <process id="Order_handling" name="Order_handling" processType="None" isClosed="false" isExecutable="false">
    <extensionElements>
      <bt:token id="Token_0avkqrt" shouldExist="false" processSnapshot="ProcessSnapshot_0yu1fss" />
    </extensionElements>
    <laneSet id="sid-b167d0d7-e761-4636-9200-76b7f0e8e83a">
      <lane id="lane">
        <flowNodeRef>start-event</flowNodeRef>
        <flowNodeRef>OD</flowNodeRef>
        <flowNodeRef>Retrieve_payment</flowNodeRef>
        <flowNodeRef>Fetch_goods</flowNodeRef>
        <flowNodeRef>SP</flowNodeRef>
        <flowNodeRef>EG</flowNodeRef>
        <flowNodeRef>PG</flowNodeRef>
      </lane>
    </laneSet>
    <startEvent id="start-event" name="Order placed">
      <outgoing>Flow_1</outgoing>
    </startEvent>
    <endEvent id="OD" name="Order delivered">
      <incoming>Flow_7</incoming>
    </endEvent>
    <serviceTask id="Retrieve_payment" name="Retrieve payment">
      <incoming>Flow_2</incoming>
      <outgoing>Flow_4</outgoing>
    </serviceTask>
    <userTask id="Fetch_goods" name="Fetch goods">
      <incoming>Flow_3</incoming>
      <outgoing>Flow_5</outgoing>
    </userTask>
    <userTask id="SP" name="Ship goods">
      <incoming>Flow_6</incoming>
      <outgoing>Flow_7</outgoing>
    </userTask>
    <sequenceFlow id="Flow_1" name="1" sourceRef="start-event" targetRef="EG" />
    <sequenceFlow id="Flow_2" name="2" sourceRef="EG" targetRef="Retrieve_payment" />
    <sequenceFlow id="Flow_3" name="3" sourceRef="EG" targetRef="Fetch_goods" />
    <sequenceFlow id="Flow_7" name="7" sourceRef="SP" targetRef="OD" />
    <sequenceFlow id="Flow_4" name="4" sourceRef="Retrieve_payment" targetRef="PG" />
    <sequenceFlow id="Flow_5" name="5" sourceRef="Fetch_goods" targetRef="PG" />
    <sequenceFlow id="Flow_6" name="6" sourceRef="PG" targetRef="SP" />
    <exclusiveGateway id="EG" name="EG">
      <incoming>Flow_1</incoming>
      <outgoing>Flow_2</outgoing>
      <outgoing>Flow_3</outgoing>
    </exclusiveGateway>
    <parallelGateway id="PG" name="PG">
      <incoming>Flow_4</incoming>
      <incoming>Flow_5</incoming>
      <outgoing>Flow_6</outgoing>
    </parallelGateway>
    <association id="Association_1gubk35" sourceRef="SP" />
  </process>
  <bpmndi:BPMNDiagram id="sid-74620812-92c4-44e5-949c-aa47393d3830">
    <bpmndi:BPMNPlane id="sid-cdcae759-2af7-4a6d-bd02-53f3352a731d" bpmnElement="Collaboration">
      <bpmndi:BPMNShape id="sid-87F4C1D6-25E1-4A45-9DA7-AD945993D06F_gui" bpmnElement="Order_handling_p" isHorizontal="true">
        <omgdc:Bounds x="160" y="50" width="569" height="260" />
        <bpmndi:BPMNLabel labelStyle="sid-84cb49fd-2f7c-44fb-8950-83c3fa153d3b" />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="sid-57E4FE0D-18E4-478D-BC5D-B15164E93254_gui" bpmnElement="lane" isHorizontal="true">
        <omgdc:Bounds x="190" y="50" width="539" height="260" />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="StartEvent_0l6sgn0_di" bpmnElement="start-event">
        <omgdc:Bounds x="211" y="122" width="36" height="36" />
        <bpmndi:BPMNLabel>
          <omgdc:Bounds x="198" y="159" width="64" height="14" />
        </bpmndi:BPMNLabel>
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Event_0qcvv2g_di" bpmnElement="OD">
        <omgdc:Bounds x="671" y="122" width="36" height="36" />
        <bpmndi:BPMNLabel>
          <omgdc:Bounds x="651" y="165" width="76" height="14" />
        </bpmndi:BPMNLabel>
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Activity_1oob6z8_di" bpmnElement="Retrieve_payment">
        <omgdc:Bounds x="349" y="100" width="100" height="80" />
        <bpmndi:BPMNLabel />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Activity_11kif52_di" bpmnElement="Fetch_goods">
        <omgdc:Bounds x="349" y="210" width="100" height="80" />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Activity_1w8qyby_di" bpmnElement="SP">
        <omgdc:Bounds x="549" y="100" width="100" height="80" />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Gateway_0rzxwom_di" bpmnElement="EG" isMarkerVisible="true">
        <omgdc:Bounds x="274" y="115" width="50" height="50" />
        <bpmndi:BPMNLabel>
          <omgdc:Bounds x="291" y="93" width="17" height="14" />
        </bpmndi:BPMNLabel>
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Gateway_0jjh0f5_di" bpmnElement="PG">
        <omgdc:Bounds x="474" y="115" width="50" height="50" />
        <bpmndi:BPMNLabel>
          <omgdc:Bounds x="491" y="93" width="17" height="14" />
        </bpmndi:BPMNLabel>
      </bpmndi:BPMNShape>
      <bpmndi:BPMNEdge id="Flow_0wq8dog_di" bpmnElement="Flow_1">
        <omgdi:waypoint x="247" y="140" />
        <omgdi:waypoint x="274" y="140" />
        <bpmndi:BPMNLabel>
          <omgdc:Bounds x="257" y="122" width="7" height="14" />
        </bpmndi:BPMNLabel>
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_0u9a0g3_di" bpmnElement="Flow_2">
        <omgdi:waypoint x="324" y="140" />
        <omgdi:waypoint x="349" y="140" />
        <bpmndi:BPMNLabel>
          <omgdc:Bounds x="333" y="122" width="7" height="14" />
        </bpmndi:BPMNLabel>
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_1mtm8jg_di" bpmnElement="Flow_3">
        <omgdi:waypoint x="299" y="165" />
        <omgdi:waypoint x="299" y="250" />
        <omgdi:waypoint x="349" y="250" />
        <bpmndi:BPMNLabel>
          <omgdc:Bounds x="319" y="229" width="7" height="14" />
        </bpmndi:BPMNLabel>
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_14fhivy_di" bpmnElement="Flow_7">
        <omgdi:waypoint x="649" y="140" />
        <omgdi:waypoint x="671" y="140" />
        <bpmndi:BPMNLabel>
          <omgdc:Bounds x="657" y="122" width="7" height="14" />
        </bpmndi:BPMNLabel>
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_1flhoxp_di" bpmnElement="Flow_4">
        <omgdi:waypoint x="449" y="140" />
        <omgdi:waypoint x="474" y="140" />
        <bpmndi:BPMNLabel>
          <omgdc:Bounds x="458" y="122" width="7" height="14" />
        </bpmndi:BPMNLabel>
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_1n9ng49_di" bpmnElement="Flow_5">
        <omgdi:waypoint x="449" y="250" />
        <omgdi:waypoint x="499" y="250" />
        <omgdi:waypoint x="499" y="165" />
        <bpmndi:BPMNLabel>
          <omgdc:Bounds x="471" y="232" width="7" height="14" />
        </bpmndi:BPMNLabel>
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_14i9c18_di" bpmnElement="Flow_6">
        <omgdi:waypoint x="524" y="140" />
        <omgdi:waypoint x="549" y="140" />
        <bpmndi:BPMNLabel>
          <omgdc:Bounds x="533" y="122" width="7" height="14" />
        </bpmndi:BPMNLabel>
      </bpmndi:BPMNEdge>
    </bpmndi:BPMNPlane>
    <bpmndi:BPMNLabelStyle id="sid-e0502d32-f8d1-41cf-9c4a-cbb49fecf581">
      <omgdc:Font name="Arial" size="11" isBold="false" isItalic="false" isUnderline="false" isStrikeThrough="false" />
    </bpmndi:BPMNLabelStyle>
    <bpmndi:BPMNLabelStyle id="sid-84cb49fd-2f7c-44fb-8950-83c3fa153d3b">
      <omgdc:Font name="Arial" size="12" isBold="false" isItalic="false" isUnderline="false" isStrikeThrough="false" />
    </bpmndi:BPMNLabelStyle>
  </bpmndi:BPMNDiagram>
</definitions>
`,orderHandlingSynchronization:`<?xml version="1.0" encoding="UTF-8"?>
<definitions xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL" xmlns:bpmndi="http://www.omg.org/spec/BPMN/20100524/DI" xmlns:omgdc="http://www.omg.org/spec/DD/20100524/DC" xmlns:omgdi="http://www.omg.org/spec/DD/20100524/DI" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" targetNamespace="" xsi:schemaLocation="http://www.omg.org/spec/BPMN/20100524/MODEL http://www.omg.org/spec/BPMN/2.0/20100501/BPMN20.xsd">
  <collaboration id="Collaboration">
    <participant id="Customer" name="Order handling" processRef="Customer_1" />
  </collaboration>
  <process id="Customer_1" name="Customer" processType="None" isClosed="false" isExecutable="false">
    <extensionElements />
    <laneSet id="sid-b167d0d7-e761-4636-9200-76b7f0e8e83a">
      <lane id="lane">
        <flowNodeRef>start-event</flowNodeRef>
        <flowNodeRef>Event_0qcvv2g</flowNodeRef>
        <flowNodeRef>Activity_0lgvp3u</flowNodeRef>
        <flowNodeRef>Gateway_1x8m4ws</flowNodeRef>
        <flowNodeRef>Activity_1up8xq1</flowNodeRef>
        <flowNodeRef>Activity_1jgyh05</flowNodeRef>
        <flowNodeRef>Gateway_0eef44j</flowNodeRef>
      </lane>
    </laneSet>
    <startEvent id="start-event" name="Order placed">
      <outgoing>Flow_0wq8dog</outgoing>
    </startEvent>
    <sequenceFlow id="Flow_0wq8dog" sourceRef="start-event" targetRef="Gateway_0eef44j" />
    <sequenceFlow id="Flow_0u9a0g3" sourceRef="Gateway_0eef44j" targetRef="Activity_1jgyh05" />
    <sequenceFlow id="Flow_1mtm8jg" sourceRef="Gateway_0eef44j" targetRef="Activity_1up8xq1" />
    <sequenceFlow id="Flow_1flhoxp" sourceRef="Activity_1jgyh05" targetRef="Gateway_1x8m4ws" />
    <sequenceFlow id="Flow_1n9ng49" sourceRef="Activity_1up8xq1" targetRef="Gateway_1x8m4ws" />
    <sequenceFlow id="Flow_14i9c18" sourceRef="Gateway_1x8m4ws" targetRef="Activity_0lgvp3u" />
    <sequenceFlow id="Flow_14fhivy" sourceRef="Activity_0lgvp3u" targetRef="Event_0qcvv2g" />
    <endEvent id="Event_0qcvv2g" name="Order delivered">
      <incoming>Flow_14fhivy</incoming>
    </endEvent>
    <userTask id="Activity_0lgvp3u" name="Ship goods">
      <incoming>Flow_14i9c18</incoming>
      <outgoing>Flow_14fhivy</outgoing>
    </userTask>
    <exclusiveGateway id="Gateway_1x8m4ws">
      <incoming>Flow_1flhoxp</incoming>
      <incoming>Flow_1n9ng49</incoming>
      <outgoing>Flow_14i9c18</outgoing>
    </exclusiveGateway>
    <userTask id="Activity_1up8xq1" name="Fetch goods">
      <incoming>Flow_1mtm8jg</incoming>
      <outgoing>Flow_1n9ng49</outgoing>
    </userTask>
    <serviceTask id="Activity_1jgyh05" name="Retrieve payment">
      <incoming>Flow_0u9a0g3</incoming>
      <outgoing>Flow_1flhoxp</outgoing>
    </serviceTask>
    <parallelGateway id="Gateway_0eef44j">
      <incoming>Flow_0wq8dog</incoming>
      <outgoing>Flow_0u9a0g3</outgoing>
      <outgoing>Flow_1mtm8jg</outgoing>
    </parallelGateway>
  </process>
  <bpmndi:BPMNDiagram id="sid-74620812-92c4-44e5-949c-aa47393d3830">
    <bpmndi:BPMNPlane id="sid-cdcae759-2af7-4a6d-bd02-53f3352a731d" bpmnElement="Collaboration">
      <bpmndi:BPMNShape id="sid-87F4C1D6-25E1-4A45-9DA7-AD945993D06F_gui" bpmnElement="Customer" isHorizontal="true">
        <omgdc:Bounds x="170" y="30" width="720" height="260" />
        <bpmndi:BPMNLabel labelStyle="sid-84cb49fd-2f7c-44fb-8950-83c3fa153d3b" />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="sid-57E4FE0D-18E4-478D-BC5D-B15164E93254_gui" bpmnElement="lane" isHorizontal="true">
        <omgdc:Bounds x="200" y="30" width="690" height="260" />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="StartEvent_0l6sgn0_di" bpmnElement="start-event">
        <omgdc:Bounds x="221" y="102" width="36" height="36" />
        <bpmndi:BPMNLabel>
          <omgdc:Bounds x="208" y="139" width="64" height="14" />
        </bpmndi:BPMNLabel>
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Event_0qcvv2g_di" bpmnElement="Event_0qcvv2g">
        <omgdc:Bounds x="772" y="102" width="36" height="36" />
        <bpmndi:BPMNLabel>
          <omgdc:Bounds x="752" y="145" width="76" height="14" />
        </bpmndi:BPMNLabel>
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Activity_1w8qyby_di" bpmnElement="Activity_0lgvp3u">
        <omgdc:Bounds x="630" y="80" width="100" height="80" />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Gateway_0j0fumn_di" bpmnElement="Gateway_1x8m4ws" isMarkerVisible="true">
        <omgdc:Bounds x="545" y="95" width="50" height="50" />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Activity_11kif52_di" bpmnElement="Activity_1up8xq1">
        <omgdc:Bounds x="380" y="190" width="100" height="80" />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Activity_1oob6z8_di" bpmnElement="Activity_1jgyh05">
        <omgdc:Bounds x="380" y="80" width="100" height="80" />
        <bpmndi:BPMNLabel />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNShape id="Gateway_1iluv37_di" bpmnElement="Gateway_0eef44j">
        <omgdc:Bounds x="295" y="95" width="50" height="50" />
      </bpmndi:BPMNShape>
      <bpmndi:BPMNEdge id="Flow_0wq8dog_di" bpmnElement="Flow_0wq8dog">
        <omgdi:waypoint x="257" y="120" />
        <omgdi:waypoint x="295" y="120" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_0u9a0g3_di" bpmnElement="Flow_0u9a0g3">
        <omgdi:waypoint x="345" y="120" />
        <omgdi:waypoint x="380" y="120" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_1mtm8jg_di" bpmnElement="Flow_1mtm8jg">
        <omgdi:waypoint x="320" y="145" />
        <omgdi:waypoint x="320" y="230" />
        <omgdi:waypoint x="380" y="230" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_1flhoxp_di" bpmnElement="Flow_1flhoxp">
        <omgdi:waypoint x="480" y="120" />
        <omgdi:waypoint x="545" y="120" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_1n9ng49_di" bpmnElement="Flow_1n9ng49">
        <omgdi:waypoint x="480" y="230" />
        <omgdi:waypoint x="570" y="230" />
        <omgdi:waypoint x="570" y="145" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_14i9c18_di" bpmnElement="Flow_14i9c18">
        <omgdi:waypoint x="595" y="120" />
        <omgdi:waypoint x="630" y="120" />
      </bpmndi:BPMNEdge>
      <bpmndi:BPMNEdge id="Flow_14fhivy_di" bpmnElement="Flow_14fhivy">
        <omgdi:waypoint x="730" y="120" />
        <omgdi:waypoint x="772" y="120" />
      </bpmndi:BPMNEdge>
    </bpmndi:BPMNPlane>
    <bpmndi:BPMNLabelStyle id="sid-e0502d32-f8d1-41cf-9c4a-cbb49fecf581">
      <omgdc:Font name="Arial" size="11" isBold="false" isItalic="false" isUnderline="false" isStrikeThrough="false" />
    </bpmndi:BPMNLabelStyle>
    <bpmndi:BPMNLabelStyle id="sid-84cb49fd-2f7c-44fb-8950-83c3fa153d3b">
      <omgdc:Font name="Arial" size="12" isBold="false" isItalic="false" isUnderline="false" isStrikeThrough="false" />
    </bpmndi:BPMNLabelStyle>
  </bpmndi:BPMNDiagram>
</definitions>
`};function cn(e,t){this._canvas=t,this._eventBus=e,this._init()}cn.prototype._init=function(){let e=a(`
    <label for="example-select">Change example:</label>
  `),t=a(`
    <select id="example-select" class="example-select">
      <option value="taskSplit">Implicit parallel gateway (Synchronization)</option>
      <option value="unsafeGateways" selected="selected">
        Exclusive gateway (Synchronization)
      </option>
      <option value="taskMerge">Implicit exclusive gateway (Synchronization)</option>
      <option value="deadActivity">Dead Activity</option>
      <option value="deadTasksConnected">Connected Dead Activities</option>
      <option value="deadReceiveTask">Dead Receive Task</option>
      <option value="stuck">Blocking PG (Termination)</option>
      <option value="deadMice">Blocking MICE (Termination)</option>
      <option value="starvation">Message Starvation (Termination)</option>
      <option value="livelock">Livelock (Termination)</option>
      <option value="reusedEndEvent">Reused end event (Unique End Events)</option>
      <option value="showcase">Complex scenario</option>
      <option value="poolsWithMessageFlows">Counterexample with messages</option>
      <option value="cycles">Quick fixes with cycles</option>
      <option value="orderHandling">Order handling example (No Termination)</option>
      <option value="orderHandlingSynchronization">Order handling example (No Synchronization)</option>
    </select>
  `),n=document.getElementById(`io-editing-tools-buttons`);n.prepend(t),n.prepend(e),document.getElementById(`example-select`).addEventListener(`change`,e=>{let t=X[e.currentTarget.value];this._eventBus.fire(`example.import`,{xml:t})});let r=new URLSearchParams(window.location.search).get(`model`);r&&X[r]?this._eventBus.on(`example.init`,()=>{let e=X[r];this._eventBus.fire(`example.import`,{xml:e}),document.getElementById(`example-select`).value=r}):this._eventBus.on(`example.init`,()=>{let e=X.unsafeGateways;this._eventBus.fire(`example.import`,{xml:e})})},cn.$inject=[`eventBus`,`canvas`];var Z=new te({container:`#canvas`,additionalModules:[sn,{__init__:[`analysisExamples`],analysisExamples:[`type`,cn]}]});function ln(e){e.requestFullscreen()}function un(){document.exitFullscreen()}var Q={fullScreen:!1,keyboardHelp:!1};document.getElementById(`js-toggle-fullscreen`).addEventListener(`click`,function(){Q.fullScreen=!Q.fullScreen,Q.fullScreen?ln(document.documentElement):un()}),document.getElementById(`js-toggle-keyboard-help`).addEventListener(`click`,function(){Q.keyboardHelp=!Q.keyboardHelp;let e=`none`;Q.keyboardHelp&&(e=`block`),document.getElementById(`io-dialog-main`).style.display=e}),document.getElementById(`io-dialog-main`).addEventListener(`click`,function(){Q.keyboardHelp=!Q.keyboardHelp,Q.keyboardHelp||(document.getElementById(`io-dialog-main`).style.display=`none`)});function dn(e,t){if(!e)return;let n=new FileReader;n.onload=function(e){let n=e.target.result;t(n)},n.readAsText(e)}var $=document.createElement(`input`);$.setAttribute(`type`,`file`),$.style.display=`none`,document.body.appendChild($),$.addEventListener(`change`,function(e){dn(e.target.files[0],fn)});function fn(e){Z.importXML(e).catch(function(e){if(e)return console.error(`could not import xml`,e)})}function pn(){return Z.saveSVG()}function mn(){return Z.saveXML({format:!0})}var hn=document.getElementById(`js-download-board`),gn=document.getElementById(`js-download-svg`),_n=document.getElementById(`js-open-new`),vn=document.getElementById(`js-open-board`);function yn(e,t,n){let r=encodeURIComponent(n);n?(e.classList.add(`active`),e.setAttribute(`href`,`data:application/xml;charset=UTF-8,`+r),e.setAttribute(`download`,t)):e.classList.remove(`active`)}var bn=xn(function(){pn().then(function(e){yn(gn,`bpmn.svg`,e.svg)}),mn().then(function(e){yn(hn,`bpmn.bpmn`,e.xml),Z.get(`eventBus`).fire(`analysis.start`,e)})},500);Z.on(`commandStack.changed`,bn),Z.on(`import.done`,bn),Z.on(`example.import`,e=>fn(e.xml)),_n.addEventListener(`click`,function(){fn(ne)}),vn.addEventListener(`click`,function(){$.value=``,$.click()}),Z.get(`eventBus`).fire(`example.init`,{});function xn(e,t){let n;return function(){n&&clearTimeout(n),n=setTimeout(e,t)}}