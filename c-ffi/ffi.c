#include <stdio.h>
#include <string.h>

char str[80];

extern char *concat_string(char *input)
{
  strcat(str, "hello");
  strcat(str, input);
  return str;
}
