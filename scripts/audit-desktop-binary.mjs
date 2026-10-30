// Check deployment imports directly from the final PE; no application execution.
import fs from'node:fs';import crypto from'node:crypto';
const path=process.argv[2];if(!path)throw Error('Supply the packaged executable path');const b=fs.readFileSync(path),pe=b.readUInt32LE(0x3c),opt=pe+24;
if(b.toString('ascii',pe,pe+4)!=='PE\0\0'||b.readUInt16LE(pe+4)!==0x8664||b.readUInt16LE(opt)!==0x20b)throw Error('Expected x64 PE32+');
const sections=Array.from({length:b.readUInt16LE(pe+6)},(_,i)=>{const s=opt+b.readUInt16LE(pe+20)+i*40;return{va:b.readUInt32LE(s+12),size:b.readUInt32LE(s+16),raw:b.readUInt32LE(s+20)};});
function file(rva){const s=sections.find(s=>rva>=s.va&&rva<s.va+s.size);if(!s)throw Error('Invalid PE RVA');return s.raw+rva-s.va;}
const imports=[];let descriptor=file(b.readUInt32LE(opt+120));for(;b.readUInt32LE(descriptor+12);descriptor+=20){const start=file(b.readUInt32LE(descriptor+12)),end=b.indexOf(0,start);imports.push(b.toString('ascii',start,end));}
const system=new Set(['advapi32.dll','bcrypt.dll','comctl32.dll','comdlg32.dll','crypt32.dll','dbghelp.dll','dwmapi.dll','gdi32.dll','gdiplus.dll','imm32.dll','kernel32.dll','msvcrt.dll','ntdll.dll','ole32.dll','oleaut32.dll','secur32.dll','shell32.dll','shlwapi.dll','user32.dll','userenv.dll','version.dll','winmm.dll','winspool.drv','ws2_32.dll','ucrtbase.dll']);
system.add('bcryptprimitives.dll');
for(const dll of imports)if(!system.has(dll.toLowerCase())&&!/^(api-ms-win-|ext-ms-win-)/i.test(dll))throw Error('Non-system runtime dependency: '+dll);
if(b.readUInt16LE(opt+68)!==2)throw Error('Expected Windows GUI subsystem');
console.log(JSON.stringify({file:path,bytes:b.length,machine:'x64',subsystem:'Windows GUI',imports,only_windows_system_dlls:true,sha256:crypto.createHash('sha256').update(b).digest('hex')},null,2));
