#include <stdio.h>
#include <stdint.h>

int foo = 1;
uint8_t sub_402130(void)
{
    foo = (foo + (((foo << 3) - foo) * 0x190 + foo) * 6) & 0x7fffffff;
    return ((foo >> 0x10) + foo) & 0xff;
}

uint8_t magi = 0x4c;
uint8_t decrypt(uint8_t inp)
{
    // uint8_t magi = 0x4c;
    // uint8_t magi = 0xee;
    uint8_t al, bl, cl, dl;
    al = 0;
    bl = 0;
    cl = 0;
    dl = 0;
    al = sub_402130();
    cl = inp;
    cl -= al;   //  sub     cl, al
    al = cl;    //  mov     al, cl
    dl = cl;    //  mov     dl, cl
    al &= 0x40; //  and     al, 0x40
    bl = cl;    //  mov     bl, cl
    dl >>= 0x2; //  shr     dl, 0x2
    al |= dl;   //  or      al, dl
    dl = cl;    //  mov     dl, cl
    al >>= 0x2; //  shr     al, 0x2
    dl &= 0x20; //  and     dl, 0x20
    al |= dl;   //  or      al, dl
    dl = cl;    //  mov     dl, cl
    al >>= 0x2; //  shr     al, 0x2
    dl &= 0x10; //  and     dl, 0x10
    al |= dl;   //  or      al, dl
    dl = cl;    //
    dl &= 0x2;  //
    bl <<= 0x2; //
    dl |= bl;   //
    bl = cl;    //
    dl <<= 0x2; //
    bl &= 4;    //
    dl |= bl;   //
    bl = cl;    //
    dl <<= 0x2; //
    bl &= 0x8;  //
    cl -= 0x5a; //
    uint8_t ret = cl;
    dl |= bl;
    al >>= 0x1;
    dl <<= 0x1;
    al |= dl;
    bl = magi;
    bl |= al;
    // printf("al=0x%x, bl=0x%x, cl=0x%x, dl=0x%x\n", al, bl, cl, dl);
    return ret;
}

#define BUFFER_SIZE (1024 * 1024)

int main(void)
{
    FILE *fin = fopen("DateBase.dat", "rb");
    FILE *fout = fopen("DateBase.bin", "wb");
    char buffer[BUFFER_SIZE];
    size_t bytes;

    while (0 < (bytes = fread(buffer, 1, sizeof(buffer), fin)))
    {
        for (int i = 0; i < bytes; i++)
        {
            buffer[i] = decrypt(buffer[i]);
        }
        fwrite(buffer, 1, bytes, fout);
    }
    fclose(fin);
    fclose(fout);
    return 0;
}
// gcc decode.c -o decode && ./decode