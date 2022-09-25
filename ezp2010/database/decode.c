#include <stdio.h>
#include <stdint.h>

#define BUFFER_SIZE (1024 * 1024)

int main(void)
{
    FILE *fin = fopen("DateBase.dat", "rb");
    FILE *fout = fopen("DateBase.bin", "wb");
    char buffer[BUFFER_SIZE];
    size_t bytes;
    int32_t k = 1;
    while (0 < (bytes = fread(buffer, 1, sizeof(buffer), fin)))
    {
        for (int i = 0; i < bytes; i++)
        {
            k = k + (((k << 3) - k) * 0x190 + k) * 6;
            buffer[i] = buffer[i] - ((k >> 0x10) + k) - 0x5a;
        }
        fwrite(buffer, 1, bytes, fout);
    }
    fclose(fin);
    fclose(fout);
    return 0;
}
// gcc decode.c -o decode && ./decode