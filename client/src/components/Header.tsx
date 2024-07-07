import { Link } from "wouter";
import Context from "../Context";

/**
 * The optional login buttons
 */
const Buttons = () => (
  <div>
    <span className="header-link-container">
      <Link href="log-in">
        <span className="header-link-text">Log In</span>
      </Link>
    </span>
    <span className="header-link-container">
      <Link href="sign-up">
        <span className="header-link-text">Sign Up</span>
      </Link>
    </span>
  </div>
);

const Header = () => {
  const context = Context;
  return (
    <header>
      <Link id="header-title" href="/">
        <span className="header-title-large">Pa</span>
        <span className="header-title-small">per</span>
        <span className="header-title-large">Tra</span>
        <span className="header-title-small">der</span>
        <span className="header-title-large">S</span>
        <span className="header-title-small">olo</span>
      </Link>
      {context.logged_in ? null : <Buttons />}
    </header>
  );
};

export default Header;
