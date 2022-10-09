import { log } from "./logger.js";

const kernel32 = {
    CreateFileA: Module.getExportByName('kernel32.dll', 'CreateFileA'),
    ReadFile: Module.getExportByName('kernel32.dll', 'ReadFile'),
    WriteFile: Module.getExportByName('kernel32.dll', 'WriteFile')
    //KERNEL32:lstrcpyA)
    //KERNEL32:lstrcmpiA)
    // KERNEL32:lstrcatA
    // KERNEL32:lstrlenA)
    //lstrcpynA
    //lstrcmpA
    //CompareStringA
    //CompareStringW)

};

const user32 = {
    MessageBoxA: Module.getExportByName('user32.dll', 'MessageBoxA'),
    SendMessageA: Module.getExportByName('user32.dll', 'SendMessageA')
};
const comdlg32 = {
    GetOpenFileNameA: Module.getExportByName('comdlg32.dll', 'GetOpenFileNameA'),
}



// const header = Memory.alloc(16);
// header
//     .writeU32(0xdeadbeef).add(4)
//     .writeU32(0xd00ff00d).add(4)
//     .writeU64(uint64("0x1122334455667788"));
// log(hexdump(header.readByteArray(16) as ArrayBuffer, { ansi: true }));
let fileDescriptors = new Map<number, string>()

Interceptor.attach(kernel32.CreateFileA, {
    onEnter(args) {
        this.filename = args[0].readCString();
    },
    onLeave(retval) {
        const ret = retval.toInt32();
        if (ret != 0) {
            fileDescriptors.set(ret, this.filename);
            //send({ func: 'CreateFileA', filename: this.filename, fd: ret });
        }
    }
});

Interceptor.attach(kernel32.ReadFile, {
    onEnter(args) {
        this.hFile = new NativePointer(args[0]);
        this.lpBuffer = new NativePointer(args[1]);
        this.nNumberOfBytesToRead = args[2].toInt32();
        this.lpNumberOfBytesRead = new NativePointer(args[3]);
        this.lpOverlapped = new NativePointer(args[4]);
        // log(`ReadFile(handle=${this.hFile},`
        //     + `lpBuffer=${this.lpBuffer}, `
        //     + `nNumberOfBytesToRead=${this.nNumberOfBytesToRead}, `
        //     + `lpNumberOfBytesRead=${this.lpNumberOfBytesRead})`);
        log(`ReadFile, num=${this.nNumberOfBytesToRead}, file=${fileDescriptors.get(this.hFile.toInt32())}`);
        // console.log('ReadFile called from:\n' +
        //     Thread.backtrace(this.context, Backtracer.FUZZY)
        //         .map(DebugSymbol.fromAddress).join('\n') + '\n');
    },
    onLeave(retval) {
        var ret = retval.toInt32();
        if (ret != 0) {
            const read = this.lpNumberOfBytesRead.isNull() ? this.nNumberOfBytesToRead : this.lpNumberOfBytesRead.readInt();
            const data = this.lpBuffer.readByteArray(read);
            log(hexdump(data as ArrayBuffer, { ansi: true, length: 64 }));
            //send({ func: 'ReadFile', filename: fileDescriptors.get(this.hFile.toInt32()) }, data);
        }
    }
});

Interceptor.attach(kernel32.WriteFile, {
    onEnter(args) {
        this.hFile = new NativePointer(args[0]);
        this.lpBuffer = new NativePointer(args[1]);
        this.nNumberOfBytesToWrite = args[2].toInt32();
        this.lpNumberOfBytesWritten = new NativePointer(args[3]);
        this.lpOverlapped = new NativePointer(args[4]);
        const data = this.lpBuffer.readByteArray(this.nNumberOfBytesToWrite);
        log(`WriteFile, num=${this.nNumberOfBytesToWrite}, , file=${fileDescriptors.get(this.hFile.toInt32())}`);
        // console.log('WriteFile called from:\n' +
        //     Thread.backtrace(this.context, Backtracer.FUZZY)
        //         .map(DebugSymbol.fromAddress).join('\n') + '\n');

        log(hexdump(data as ArrayBuffer, { ansi: true, length: 64 }));
        //send({ func: 'WriteFile', filename: fileDescriptors.get(this.hFile.toInt32()) }, data);
    },
    onLeave(retval) {
    }
});