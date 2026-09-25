char read_char() {
    read_white();
    int c = getchar();
    if (c == EOF) halt();
    return c;
}
