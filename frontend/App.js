import "regenerator-runtime/runtime";
import React from "react";

import "./assets/css/global.css";

import { login, logout, new_registry, get } from "./assets/js/near/utils";
import getConfig from "./assets/js/near/config";

export default function App() {
  // if not signed in, return early with sign-in prompt
  if (!window.walletConnection.isSignedIn()) {
    return (
      <main>
        <h1>
          <label
            htmlFor="greeting"
            style={{
              color: "var(--secondary)",
              borderBottom: "2px solid var(--secondary)",
            }}
          >
            {""}
          </label>
          ! Welcome to NEAR!
        </h1>
        <p>
          Your contract is storing a greeting message in the NEAR blockchain. To
          change it you need to sign in using the NEAR Wallet. It is very
          simple, just use the button below.
        </p>
        <p>
          Do not worry, this app runs in the test network ("testnet"). It works
          just like the main network ("mainnet"), but using NEAR Tokens that are
          only for testing!
        </p>
        <p style={{ textAlign: "center", marginTop: "2.5em" }}>
          <button onClick={login}>Sign in</button>
        </p>
      </main>
    );
  }

  return (
    // use React Fragment, <>, to avoid wrapping elements in unnecessary divs
    <>
      <button className="link" style={{ float: "right" }} onClick={logout}>
        Sign out
      </button>
      <main>
        <h1>
          <label
            htmlFor="greeting"
            style={{
              color: "var(--secondary)",
              borderBottom: "2px solid var(--secondary)",
            }}
          >
            {""}
          </label>
          {
            " " /* React trims whitespace around tags; insert literal space character when needed */
          }
          {window.accountId}!
        </h1>

        <p>
          Look at that! A Hello World app! This greeting is stored on the NEAR
          blockchain. Check it out:
        </p>
        <ol>
          <li>
            Look in <code>src/App.js</code> and <code>src/utils.js</code> –
            you'll see <code>get_greeting</code> and <code>set_greeting</code>{" "}
            being called on <code>contract</code>. What's this?
          </li>
          <li>
            Ultimately, this <code>contract</code> code is defined in{" "}
            <code>assembly/main.ts</code> – this is the source code for your{" "}
            <a
              target="_blank"
              rel="noreferrer"
              href="https://docs.near.org/docs/develop/contracts/overview"
            >
              smart contract
            </a>
            .
          </li>
          <li>
            When you run <code>yarn dev</code>, the code in{" "}
            <code>assembly/main.ts</code> gets deployed to the NEAR testnet. You
            can see how this happens by looking in <code>package.json</code> at
            the <code>scripts</code> section to find the <code>dev</code>{" "}
            command.
          </li>
        </ol>
        <hr />
        <p>
          To keep learning, check out{" "}
          <a target="_blank" rel="noreferrer" href="https://docs.near.org">
            the NEAR docs
          </a>{" "}
          or look through some{" "}
          <a target="_blank" rel="noreferrer" href="https://examples.near.org">
            example apps
          </a>
          .
        </p>
        <button onClick={new_registry}>AAAAAAAA</button>
        <button onClick={get}>BBBBBBB</button>
      </main>
    </>
  );
}
