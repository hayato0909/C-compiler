テストについて

・test.sh
	いくつかの計算のテストを実行するシェル

・test1
	関数呼び出しに関する実行ファイル
	" {foo();} "という入力を与えたコンパイラの結果と以下のcプログラムをcc -cによりコンパイルしたオブジェクトファイルをリンクしたもの。
	------
	#include <stdio.h>

	int foo() {
		printf("OK\n");
	}
	------

・test2, 3
	関数呼び出しに関する実行ファイル
	test1と違い引数を与えたもの。
	コンパイラへの入力はそれぞれ以下の通り
		test2: "{ foo(3, 4); }"
		test3: "{ foo(3, 4+4-2); }"
	また、リンクに用いたcファイルは以下の通り
	------
	#include <stdio.h>

	int foo(int a, int b) {
		printf("%d + %d = %d\n", a, b, a+b);
	}
	------
