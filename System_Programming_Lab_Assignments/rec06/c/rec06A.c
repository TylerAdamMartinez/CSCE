#include <stdlib.h>
#include <stdio.h>
#include <unistd.h>

int main() {
  pid_t pid;
  pid = fork();

  if (pid == 0) {
    exit(0);
  }
  else if (pid > 0) {
    sleep(5);
    system("ps -e -o pid,ppid,stat,user,cmd | grep $USER");
  }
  else {
    perror("fork error\n");
  }

}
