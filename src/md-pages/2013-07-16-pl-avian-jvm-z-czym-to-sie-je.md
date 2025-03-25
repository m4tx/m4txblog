---
title: 'Avian JVM — z czym to się je'
permalink: 'avian-jvm-z-czym-to-sie-je'
category: 'Porady'
tags: ['c++', 'alternatywa', 'alternatywy', 'java', 'JVM', 'Avian JVM', 'Wirtualna maszyna', 'OpenJDK']
language: pl
date: 2013-07-16 18:04:11+0000
---

**Zapewne każdy (a przynajmniej większość) odwiedzający m4txbloga wie, że jest coś takiego jak Java i że programy napisane w niej uruchamia się za pomocą wirtualnej maszyny Javy (w skrócie JVM); większość ma także taką maszynę wirtualną zainstalowaną i w zdecydowanej większości przypadków jest to Oracle JVM, bądź OpenJDK. Nie każdy jednak wie, że są alternatywy. Lepsze alternatywy.**

**Mowa oczywiście o tytułowym Avian JVM.**

Czymże jednak jest ten Avian JVM? Z [artykułu na anglojęzycznej Wikipedii](http://en.wikipedia.org/wiki/Avian_JVM) dowiadujemy się, że Avian to otwarta i lekka implementacja maszyny wirtualnej Javy. Nie byłoby w niej jednak nic szczególnego (w końcu otwartą implementacją jest również OpenJDK, który jest, swoją drogą, [implementacją wzorcową dla maszyn wirtualnych Javy](https://blogs.oracle.com/henrik/entry/moving_to_openjdk_as_the)). Avian JVM pozwala jednak na coś, na co nie pozwala żadna z dwóch najpopularniejszych implementacji JVM: **pozwala tworzyć samowystarczalne pliki wykonywalne aplikacji**. Daje to więc szereg nowych możliwości:

- Pozwala tworzyć aplikacje które nie wymagają zainstalowanej maszyny wirtualnej Javy w systemie,
- Pozwala na dostarczanie aplikacji tam, gdzie zainstalowanie JVM-a nie jest możliwe — najpopularniejszym przykładem jest tu chyba system iOS,
- Pozwala zmniejszyć ilość danych koniecznych do uruchomienia aplikacji — zamiast aplikacji i wirtualnej maszyny Javy (która "waży" przecież swoje, bo około 70 MB) dostarczamy wszystko w jednym pliku, a maszyna wirtualna jest dostosowana do naszej aplikacji (można bowiem usunąć z biblioteki standardowej wszystko to, co nie jest w danej aplikacji potrzebne, a biorąc pod uwagę obszerność i wszechstronność domyślnej biblioteki standardowej Javy, zyskujemy całkiem sporo),
- Zwiększa nieco szybkość działania (a zwłaszcza uruchamiania) aplikacji, a także minimalnie zmniejsza zużycie pamięci.

Oprócz tego Avian JVM stosuje podstawowe techniki optymalizacyjne tak jak najpopularniejsze implementacje JVM — nie zabrakło więc m.in. kompilatora JIT.

Brzmi wspaniale? Oczywiście! Przejdźmy więc do części praktycznej wpisu...

## Pobieranie i podstawowa konfiguracja Avian JVM

Pierwszym, co należy zrobić by móc poobcować z Avian JVM, jest oczywiście pobranie go. Najprościej zrobić to poprzez sklonowanie repozytorium gita (za pomocą komendy `git clone https://github.com/ReadyTalk/avian.git`) bądź poprzez pobranie ZIP-a z [repozytorium na githubie](https://github.com/ReadyTalk/avian).

(Uprzedzam także od razu, że korzystam z Linuksa, konkretniej Ubuntu 13.04 64-bit, więc prezentowane tutaj komendy będą przeznaczone właśnie na Linuksa)

Najpierw potrzebowali będziemy kompilatora i bibliotek niezbędnych do kompilacji Aviana. Wydajemy więc polecenie:

```bash
sudo apt-get install g++ zlib1g-dev openjdk-7-jdk
````

Następnie musimy podać ścieżkę do **JDK** oraz skompilować maszynę wirtualną:

```bash
export JAVA_HOME=/usr/lib/jvm/java-7-openjdk-amd64/ # W przypadku, gdy mam OpenJDK w wersji 7; w przeciwnym razie należy samemu zlokalizować miejsce instalacji JDK
make
```

Testujemy działanie Avian JVM:

```plain
$ build/linux-x86_64/avian -cp build/linux-x86_64/test Hello
hello, world!
```

Osoby, które z wirtualnymi maszynami Javy mają kontakt nie od dziś, z pewnością zauważą znacznie krótszy czas uruchamiania Aviana niż OpenJDK. Małe (i mało obiektywne co prawda, ale jednak) porównanie:

```plain
$ time build/linux-x86_64/avian -cp build/linux-x86_64/test Hello
hello, world!

real    0m0.008s
user    0m0.000s
sys    0m0.004s
$ time java -cp build/linux-x86_64/test Hello
hello, world!

real    0m0.072s
user    0m0.060s
sys    0m0.012s
```

(Porównanie Aviana 0.7 i OpenJDK 1.7.0_21)

Jak więc widać, różnica jest dość duża. Porównywanie szybkości jednak to nie temat tego wpisu — przejdźmy więc do następnej sekcji.

## Tworzenie samowystarczalnych plików wykonywalnych

Posłużymy się, tak jak i w poprzedniej części, [poradnikiem na stronie readme Aviana](https://github.com/ReadyTalk/avian#embedding).

Wpierw tworzymy więc katalog "hello" i przechodzimy do niego. Rozpakowujemy do niego zawartość libavian.a, a także kopiujemy plik classpath.jar i zmieniamy jego docelową nazwę na boot.jar.

```bash
mkdir hello
cd hello
ar x ../build/linux-x86_64/libavian.a
cp ../build/linux-x86_64/classpath.jar boot.jar
```

Po całej operacji uzyskujemy mniej-więcej taką strukturę plików:

```plain
$ ls -la
razem 29064
drwxrwxr-x 2 m4tx m4tx    4096 lip 16 17:27 .
drwxrwxr-x 9 m4tx m4tx    4096 lip 16 17:26 ..
-rw-rw-r-- 1 m4tx m4tx  579026 lip 16 17:27 boot.jar
-rw-rw-r-- 1 m4tx m4tx  818896 lip 16 17:27 build_linux-x86_64_builtin.o
-rw-rw-r-- 1 m4tx m4tx 1479064 lip 16 17:27 build_linux-x86_64_classpath-avian.o
-rw-rw-r-- 1 m4tx m4tx  140064 lip 16 17:27 build_linux-x86_64_codegen_compiler_context.o
-rw-rw-r-- 1 m4tx m4tx  902840 lip 16 17:27 build_linux-x86_64_codegen_compiler_event.o
-rw-rw-r-- 1 m4tx m4tx  157368 lip 16 17:27 build_linux-x86_64_codegen_compiler_frame.o
-rw-rw-r-- 1 m4tx m4tx  143912 lip 16 17:27 build_linux-x86_64_codegen_compiler_ir.o
-rw-rw-r-- 1 m4tx m4tx 1349920 lip 16 17:27 build_linux-x86_64_codegen_compiler.o
-rw-rw-r-- 1 m4tx m4tx  190920 lip 16 17:27 build_linux-x86_64_codegen_compiler_promise.o
-rw-rw-r-- 1 m4tx m4tx  221208 lip 16 17:27 build_linux-x86_64_codegen_compiler_read.o
-rw-rw-r-- 1 m4tx m4tx  259544 lip 16 17:27 build_linux-x86_64_codegen_compiler_regalloc.o
-rw-rw-r-- 1 m4tx m4tx  188464 lip 16 17:27 build_linux-x86_64_codegen_compiler_resource.o
-rw-rw-r-- 1 m4tx m4tx  452584 lip 16 17:27 build_linux-x86_64_codegen_compiler_site.o
-rw-rw-r-- 1 m4tx m4tx  198376 lip 16 17:27 build_linux-x86_64_codegen_compiler_value.o
-rw-rw-r-- 1 m4tx m4tx   96536 lip 16 17:27 build_linux-x86_64_codegen_registers.o
-rw-rw-r-- 1 m4tx m4tx   96568 lip 16 17:27 build_linux-x86_64_codegen_targets.o
-rw-rw-r-- 1 m4tx m4tx  411504 lip 16 17:27 build_linux-x86_64_codegen_target_x86_assembler.o
-rw-rw-r-- 1 m4tx m4tx  132376 lip 16 17:27 build_linux-x86_64_codegen_target_x86_block.o
-rw-rw-r-- 1 m4tx m4tx  142736 lip 16 17:27 build_linux-x86_64_codegen_target_x86_context.o
-rw-rw-r-- 1 m4tx m4tx  136656 lip 16 17:27 build_linux-x86_64_codegen_target_x86_detect.o
-rw-rw-r-- 1 m4tx m4tx  549368 lip 16 17:27 build_linux-x86_64_codegen_target_x86_encode.o
-rw-rw-r-- 1 m4tx m4tx  218656 lip 16 17:27 build_linux-x86_64_codegen_target_x86_fixup.o
-rw-rw-r-- 1 m4tx m4tx  168616 lip 16 17:27 build_linux-x86_64_codegen_target_x86_multimethod.o
-rw-rw-r-- 1 m4tx m4tx  713872 lip 16 17:27 build_linux-x86_64_codegen_target_x86_operations.o
-rw-rw-r-- 1 m4tx m4tx  142968 lip 16 17:27 build_linux-x86_64_codegen_target_x86_padding.o
-rw-rw-r-- 1 m4tx m4tx 6880936 lip 16 17:27 build_linux-x86_64_compile.o
-rw-rw-r-- 1 m4tx m4tx     940 lip 16 17:27 build_linux-x86_64_compile-x86-asm.o
-rw-rw-r-- 1 m4tx m4tx  524976 lip 16 17:27 build_linux-x86_64_finder.o
-rw-rw-r-- 1 m4tx m4tx  787576 lip 16 17:27 build_linux-x86_64_heap_heap.o
-rw-rw-r-- 1 m4tx m4tx  421784 lip 16 17:27 build_linux-x86_64_java-io.o
-rw-rw-r-- 1 m4tx m4tx  421648 lip 16 17:27 build_linux-x86_64_java-lang.o
-rw-rw-r-- 1 m4tx m4tx  205912 lip 16 17:27 build_linux-x86_64_java-net.o
-rw-rw-r-- 1 m4tx m4tx  488768 lip 16 17:27 build_linux-x86_64_java-nio.o
-rw-rw-r-- 1 m4tx m4tx  170688 lip 16 17:27 build_linux-x86_64_java-util.o
-rw-rw-r-- 1 m4tx m4tx  252736 lip 16 17:27 build_linux-x86_64_java-util-zip.o
-rw-rw-r-- 1 m4tx m4tx 2746152 lip 16 17:27 build_linux-x86_64_jnienv.o
-rw-rw-r-- 1 m4tx m4tx 5397792 lip 16 17:27 build_linux-x86_64_machine.o
-rw-rw-r-- 1 m4tx m4tx  399840 lip 16 17:27 build_linux-x86_64_process.o
-rw-rw-r-- 1 m4tx m4tx  551960 lip 16 17:27 build_linux-x86_64_util.o
-rw-rw-r-- 1 m4tx m4tx  519888 lip 16 17:27 build_linux-x86_64_vm_system_posix.o
-rw-rw-r-- 1 m4tx m4tx    1157 lip 16 17:27 build_linux-x86_64_x86-asm.o
```

Następnie tworzymy plik Hello.java i piszemy kod źródłowy programu. W moim (naszym?) przypadku będzie to po prostu standardowy Hello world.

```bash
vim Hello.java
```

```java
public class Hello {
    public static void main(String args[]) {
        System.out.println("Hello, world!");
    }
}
```

Następnie kompilujemy nasz program i tworzymy plik `.JAR`:

```bash
javac -bootclasspath boot.jar Hello.java
jar u0f boot.jar Hello.class
```

...potem zaś przekształcamy plik `.JAR` w plik `.o`:

```bash
../build/linux-x86_64/binaryToObject/binaryToObject boot.jar boot-jar.o _binary_boot_jar_start _binary_boot_jar_end linux x86_64
```

Piszemy w C++ program, który uruchamia naszą maszynę wirtualną i wykonuje określoną funkcję.

```bash
vim embedded-jar-main.cpp
```

Ja oczywiście poszedłem na łatwiznę i skopiowałem kod z podanego wcześniej tutoriala... 😄

```cpp
#include "stdint.h"
#include "jni.h"
#include "stdlib.h"

#if (defined __MINGW32__) || (defined _MSC_VER)
#  define EXPORT __declspec(dllexport)
#else
#  define EXPORT __attribute__ ((visibility("default"))) \
  __attribute__ ((used))
#endif

#if (! defined __x86_64__) && ((defined __MINGW32__) || (defined _MSC_VER))
#  define SYMBOL(x) binary_boot_jar_##x
#else
#  define SYMBOL(x) _binary_boot_jar_##x
#endif

extern "C" {
    extern const uint8_t SYMBOL(start)[];
    extern const uint8_t SYMBOL(end)[];

    EXPORT const uint8_t*
    bootJar(unsigned* size)
    {
        *size = SYMBOL(end) - SYMBOL(start);
        return SYMBOL(start);
    }
}

extern "C" void __cxa_pure_virtual(void) {
    abort();
}

int main(int ac, const char** av)
{
    JavaVMInitArgs vmArgs;
    vmArgs.version = JNI_VERSION_1_2;
    vmArgs.nOptions = 1;
    vmArgs.ignoreUnrecognized = JNI_TRUE;

    JavaVMOption options[vmArgs.nOptions];
    vmArgs.options = options;

    options[0].optionString = const_cast<char*>("-Xbootclasspath:[bootJar]");

    JavaVM* vm;
    void* env;
    JNI_CreateJavaVM(&vm, &env, &vmArgs);
    JNIEnv* e = static_cast<JNIEnv*>(env);

    jclass c = e->FindClass("Hello");
    if (!e->ExceptionCheck()) {
        jmethodID m = e->GetStaticMethodID(c, "main", "([Ljava/lang/String;)V");
        if (!e->ExceptionCheck()) {
            jclass stringClass = e->FindClass("java/lang/String");
            if (!e->ExceptionCheck()) {
                jobjectArray a = e->NewObjectArray(ac-1, stringClass, 0);
                if (!e->ExceptionCheck()) {
                    for (int i = 1; i < ac; i++) {
                        e->SetObjectArrayElement(a, i-1, e->NewStringUTF(av[i]));
                    }
                    e->CallStaticVoidMethod(c, m, a);
                }
            }
        }
    }

    int exitCode = 0;
    if (e->ExceptionCheck()) {
        exitCode = -1;
        e->ExceptionDescribe();
    }

    vm->DestroyJavaVM();
    return exitCode;
}
```

Możemy więc w końcu skompilować i zlinkować nasz program!

```bash
g++ -I$JAVA_HOME/include -I$JAVA_HOME/include/linux -D_JNI_IMPLEMENTATION_ -c embedded-jar-main.cpp -o main.o
g++ -rdynamic *.o -ldl -lpthread -lz -o hello
strip --strip-all hello
```

Po wykonaniu pierwszych dwóch poleceń otrzymujemy plik wykonywalny hello o rozmiarze aż 8,3 MB. Dlatego też wykonujemy także trzecią komendę, za pomocą której wycinamy z naszego pliku wszystko to, co nie jest potrzebne do jego poprawnego wykonania. Otrzymujemy plik o rozmiarze 1,3 MB.

Na końcu sprawdzamy jeszcze, czy wszystko działa:

```plain
$ ./hello
Hello, world!
```

😄

## Zakończenie

W tym krótkim poradniku przedstawiłem po krótce zalety wirtualnej maszyny Avian oraz opisałem, w jaki sposób można utworzyć plik wykonywalny zawierający program napisany w Javie, jednak niewymagający zainstalowanej wirtualnej maszyny Javy w systemie. Oczywiście, wszystkie aspekty potraktowałem dość skrótowo, bo m.in. nie opisałem, jak użyć LZMA, by jeszcze zmniejszyć rozmiar pliku wykonywalnego, jednak wpis miał mieć charakter bardzo krótkiego i niezbyt zaawansowanego wprowadzenia do tematu "czym jest Avian JVM, oraz do czego i jak go można użyć". Być może za jakiś czas pojawią się na m4txblogu kolejne wpisy dotyczące Aviana — chociażby o tym, jak użyć zewnętrznych bibliotek, jak skompresować plik wykonywalny oraz poruszające inne ciekawe aspekty 😄

Pozdrawiam.
