use crate::syntatic_analyzer::*;

#[test]
fn test_syntax_and_semantics(){
    let code = String::from("
        type int = integer
        type intArray = array[10] of int

        type Pessoa = struct {
          nome : string;
          id : int;
          telefones : intArray
        }

        type pessoaArray = array[20] of Pessoa

        function funcaoRecursiva(n : int) : integer {
          var p : integer;
          var x : intArray;
          var pessoa : Pessoa;
          if (n == pessoa.id)
            p = funcaoRecursiva(n-1);
        }

        function addPessoa(posicao : integer, pessoa : Pessoa,arr : pessoaArray) : integer {
          var a,g,i,n : int;
          var b : int;
          var c : boolean;
          var str : string;
          var carac1, carac2 : char;

          if (a == 0)
            g = funcaoRecursiva(i-1);

          a = posicao;
          c = (a>b) && (a<b) || (a <= b) || (b>= a);

          if (a > b) {
            var tmp : integer;
            tmp = funcaoRecursiva(tmp-1);
          }
        }

        
    ");
    
    let mut parser = StateMachine::new(code);
    assert_eq!(parser.parse(), true);
}

#[test]
fn test_syntax_and_semantics_2() {
    let code = String::from("
        function fibonacci(n : integer) : integer {
          var ret : integer;
          if (n >= 2)
            ret = fibonacci(n-1) + fibonacci(n-2);
        } 
    ");

    let mut parser = StateMachine::new(code);
    assert_eq!(parser.parse(), true);
}

#[test]
#[should_panic]
fn test_syntax_and_semantics_3() {
    let code = String::from("
        function fibonacci(n : integer) : integer {
          var ret : integer;
          if (n >= 2)
            ret = fibonacci(n-1) + fibonacci(n-2);
        } 

        function main(x : integer) : boolean {
            var res : integer;
            res = fibonacci();
        }
    ");

    let mut parser = StateMachine::new(code);
    assert_eq!(parser.parse(), true);
}

#[test]
#[should_panic]
fn test_syntax_and_semantics_4() {
    let code = String::from("
        function fibonacci(n : integer) : integer {
          var ret : integer;
          if (n >= 2)
            ret = fibonacci(n-1) + fibonacci(n-2);
        } 

        function main(x : integer) : boolean {
            var res : boolean;
            res = fibonacci(5);
        }
    ");

    let mut parser = StateMachine::new(code);
    assert_eq!(parser.parse(), true);
}

#[test]
fn test_syntax_and_semantics_5() {
    let code = String::from("
        type aluno = struct {
            nome, sobrenome : string;
            id : integer
        }

        type professor = struct {
            name, sobrenome : string;
            id : integer
        }
        
        type materia = struct {
            name : string; prof : professor
        }

        function main(x : string) : integer {
            var rafael : aluno;
            var matheus : aluno;
            var daniel : professor;
            var matematica : materia;
            var resultado : integer;

            resultado = -1;

            if (rafael == matheus) 
                resultado = 0;
        }
    ");

    let mut parser = StateMachine::new(code);
    assert_eq!(parser.parse(), true);
}

#[test]
fn test_syntax_and_semantics_6() {
    let code = String::from("
        function main(n : integer) : boolean {
            var cnt : integer;
            cnt = 0;
            while ( true ) {
                var temp : integer;
                temp = cnt++;
                if (cnt == 10) {
                    var x : string;
                    break;
                }
            }
        }
    ");

    let mut parser = StateMachine::new(code);
    assert_eq!(parser.parse(), true);
}

#[test]
#[should_panic]
fn test_syntax_and_semantics_7() {
    let code = String::from("
        type casa = struct {
            morador, endereco : string
        }

        type carro = struct {
            motorista, placa : string
        }
        

        function main(x : string) : integer {
            var minha_casa : casa;
            var meu_carro : carro;
            var ok : boolean;

            if (minha_casa == meu_carro) ok = false; 
        }
    ");

    let mut parser = StateMachine::new(code);
    assert_eq!(parser.parse(), true);
}


