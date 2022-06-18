use stylist::{css, StyleSource};
pub fn login_register_style() -> StyleSource<'static> {
    css!(
        r#"
  height: 48vh;
  justify-content: center;
  align-items: center;
  text-align: center;
  margin-top: 10vh;
  margin-bottom: 10vh;
          "#
    )
}
pub fn main_menu_style() -> StyleSource<'static> {
    // 70vh because of footer
    css!(
        r#"
          display: flex;
  height: 70vh;
  justify-content: center;
  align-items: center;
  text-align: center;
        "#
    )
}
pub fn main_menu_list() -> StyleSource<'static> {
    css!(
        r#"
  display: flex;
  flex-direction: column;
  align-items: start;
  list-style-type: none;
    "#
    )
}
pub fn main_menu_button() -> StyleSource<'static> {
   css!(
        r#"
  align-items: center;
  background-color:  #fee6e3;
  border: 2px solid #111;

  border-radius: 8px;
  box-sizing: border-box;
  color: #df73ff;
  cursor: pointer;
  display: block;
  font-family: Inter,sans-serif;
  font-size: 16px;
  height: 48px;
  justify-content: center;
  line-height: 24px;
  max-width: 100%;
  padding: 0 25px;
  margin-right: 35px;
  position: relative;
  text-align: center;
  user-select: none;
  -webkit-user-select: none;
  touch-action: manipulation;
    :after {
  background-color: #111;
  border-radius: 8px;
  content: "";
  display: block;
  height: 48px;
  left: 0;
  width: 100%;
  position: absolute;
  top: -2px;
  transform: translate(8px, 8px);
  transition: transform .2s ease-out;
  z-index: -1;
    }

        :hover:after {
  transform: translate(0, 0);
}

:active {
  outline: 0;
}

:hover {
  outline: 0;
}

@media (min-width: 768px) {
  .button-56 {
    padding: 0 40px;
  }
}"#
    )
}
pub fn menu_list() -> StyleSource<'static> {
    css!(
        r#"
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  text-align: center;
"#
    )
}
pub fn button() -> StyleSource<'static> {
    css!(
        r#"
      align-items: center;
  justify-content: center;
  position: relative;
  text-align: center;
  background: none!important;
  border: none;
  padding: 0!important;
  font-family: Inter,sans-serif;
  color: black;
  cursor: pointer;
  height: 5.5vh;
  :hover {text-decoration:underline;}
  :active {text-decoration:underline;}
"#)
}
pub fn form_css() -> StyleSource<'static>{
    css!(r#"
         justify-content: center;
  align-items: center;
  text-align: center;
    "#)
}
pub fn form_elem() -> StyleSource<'static>{
    css!(r#"
        padding: 2px 2px;
        margin: 4px 0;
    "#)
}
pub fn user_msg() -> StyleSource<'static> {
    css!(r#"
            padding:4px;
            max-width:200px;
      display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  text-align: center;
    z-index:2;
    "#)
}
pub fn green_user_msg() -> StyleSource<'static>{
    css!(r#"
    background-color:Green;
    "#)
}
pub fn red_user_msg() -> StyleSource<'static> {
    css!(r#"
    background-color:Red;
    "#)

}
