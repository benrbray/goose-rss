import { ParentProps } from "solid-js";
import "./Navigation.css";
import { A } from "@solidjs/router";

const NavButton = (props: { href: string } & ParentProps) => {
  return <div class="nav-button">
    <A
      href={props.href}
      class="nav-button"
      activeClass="active"
      inactiveClass="inactive"
    >
      {props.children}
    </A>
  </div>;
}

export const Navigation = () => {
  return <nav class="navigation">
    {/* top area is static  */}
    <div class="top">
    <NavButton href="/createFeed">Create Feed</NavButton>
    <NavButton href="/manageFeeds">Manage Feeds</NavButton>
    </div>
    {/* bottom area lists feeds organized by category */}
    <div class="bottom">
      
    </div>
  </nav>;
}
