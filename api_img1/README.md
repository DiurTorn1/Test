Программа реализованна на по принципу примитивного Rest Api.

За Backend отвечает Rust, реализующий соединение с Postgresql посредством модуля Diesel.
Frontend - не сложный Javascript.

Для запуска программы необходима установленная Postgres и созданная база данных под именем: "Api".
Далее выполняем стандартную команду в консоле, в папке программы: "cargo run". Далее уважаемый модуль Rocket продоставит нам локальный доступ к бд ("Rocket has launched from http://localhost:8000"). Пройдя по ссылке на браузере мы увидим несколько input. В input text вводим url(картика должна быть где-то в интернете) картинки и жмём на input "Preview". В появившимся окне вводим имя картинке и жмём input "Post img". Проверяем успех загрузки картинки на input "Get img".
