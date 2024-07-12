import { ChangeEvent, FormEvent, useContext, useState } from "react";
import styles from "./sign-up.module.scss";
import { Context } from "../Context";
import { useLocation } from "wouter";

interface FormInfo {
  username: string;
  password: string;
}

export default function SignUp() {
  const context = useContext(Context);
  const [, setLocation] = useLocation();
  const [showPass, setShowPass] = useState(false);
  const [info, setInfo] = useState<FormInfo>({ username: "", password: "" });
  const infoChange = (event: ChangeEvent<HTMLInputElement>) => {
    setInfo({ ...info, [event.target.name]: event.target.value });
  };
  const [attempts, setAttempts] = useState(0);
  const [response, setResponse] = useState<Response | null>(null);

  const signUp = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    const res = await fetch("/api/req-sign-up", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(info),
    });
    if (res.status == 200) {
      context.auth.setLogin(true);
      setLocation("/");
    }
    setResponse(res);
    setAttempts(attempts + 1);
  };

  const passInfoBarText = () => {
    switch (response?.status) {
      case 409:
        return "Incorrect password inputted";
      default:
        return "";
    }
  };
  const nameInfoBarText = () => {
    switch (response?.status) {
      case 400:
        return "Username not found";
      default:
        return "";
    }
  };

  if (context.auth.login) {
    return (
      <div>
        <h1>User is already logged in</h1>
      </div>
    );
  }

  return (
    <div id={styles.page}>
      <form id={styles.form} onSubmit={signUp}>
        <h2>Create a patras account</h2>
        <p>Complete the form below to create your account</p>

        <div id={styles.usernameLabelDiv}>
          <label className={styles.label} htmlFor="username-input">
            Username:
          </label>
        </div>
        <input
          id={styles.usernameInput}
          className={styles.input}
          name="username"
          type="text"
          autoComplete="username"
          value={info.username}
          onChange={infoChange}
        />
        <div id={styles.nameInfoBar} className={styles.infoBar}>
          {nameInfoBarText()}
        </div>
        <div id={styles.passwordLabelDiv}>
          <label className={styles.label} htmlFor="password-input">
            Password:
          </label>
          <input
            id={styles.showPasswordInput}
            type="checkbox"
            checked={showPass}
            tabIndex={-1}
            onChange={() => setShowPass((prev) => !prev)}
          />
          <span id={styles.showPasswordSpan}>Show</span>
        </div>
        <input
          id={styles.passwordInput}
          className={styles.input}
          name="password"
          type={showPass ? "text" : "password"}
          autoComplete="password"
          value={info.password}
          onChange={infoChange}
        />
        <div id={styles.passInfoBar} className={styles.infoBar}>
          {passInfoBarText()}
        </div>
        <input id={styles.submitButton} type="submit" value="Sign Up" />
      </form>
    </div>
  );
}
