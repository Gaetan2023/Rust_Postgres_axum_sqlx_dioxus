
    meta { charset: "utf-8" }
    title { "Form" }
    link {
        href: "https://unpkg.com/tailwindcss@^1.0/dist/tailwind.min.css",
        rel: "stylesheet",
    }
    section { class: "py-6 dark:bg-gray-100 dark:text-gray-900",
        div { class: "py-8 lg:py-16 px-4 mx-auto max-w-screen-md",
            h2 { class: "mb-4 text-4xl tracking-tight font-extrabold text-center text-gray-900 dark:text-white",
                "Contact Us"
            }
            p { class: "mb-8 lg:mb-16 font-light text-center text-gray-500 dark:text-gray-400 sm:text-xl",
                "Got a technical issue? Want to send feedback about a beta feature? Need details about our Business plan? Let us know."
            }
            div { class: "grid max-w-6xl grid-cols-1 px-6 mx-auto lg:px-8 md:grid-cols-2 md:divide-x",
                div { class: "py-6 md:py-0 md:px-6",
                    h1 { class: "text-4xl font-bold", "Get in touch" }
                    p { class: "pt-2 pb-4", "Fill in the form to start a conversation" }
                    div { class: "space-y-4",
                        p { class: "flex items-center",
                            svg {
                                class: "w-5 h-5 mr-2 sm:mr-6",
                                fill: "currentColor",
                                view_box: "0 0 20 20",
                                xmlns: "http://www.w3.org/2000/svg",
                                path {
                                    clip_rule: "evenodd",
                                    d: "M5.05 4.05a7 7 0 119.9 9.9L10 18.9l-4.95-4.95a7 7 0 010-9.9zM10 11a2 2 0 100-4 2 2 0 000 4z",
                                    fill_rule: "evenodd",
                                }
                            }
                            span { "Fake address, 9999 City" }
                        }
                        p { class: "flex items-center",
                            svg {
                                class: "w-5 h-5 mr-2 sm:mr-6",
                                fill: "currentColor",
                                view_box: "0 0 20 20",
                                xmlns: "http://www.w3.org/2000/svg",
                                path { d: "M2 3a1 1 0 011-1h2.153a1 1 0 01.986.836l.74 4.435a1 1 0 01-.54 1.06l-1.548.773a11.037 11.037 0 006.105 6.105l.774-1.548a1 1 0 011.059-.54l4.435.74a1 1 0 01.836.986V17a1 1 0 01-1 1h-2C7.82 18 2 12.18 2 5V3z" }
                            }
                            span { "123456789" }
                        }
                        p { class: "flex items-center",
                            svg {
                                class: "w-5 h-5 mr-2 sm:mr-6",
                                fill: "currentColor",
                                view_box: "0 0 20 20",
                                xmlns: "http://www.w3.org/2000/svg",
                                path { d: "M2.003 5.884L10 9.882l7.997-3.998A2 2 0 0016 4H4a2 2 0 00-1.997 1.884z" }
                                path { d: "M18 8.118l-8 4-8-4V14a2 2 0 002 2h12a2 2 0 002-2V8.118z" }
                            }
                            span { "contact@business.com" }
                        }
                    }
                }
                form {
                    action: "/submit",
                    class: "flex flex-col py-6 space-y-6 md:py-0 md:px-6",
                    method: "post",
                    novalidate: "",
                    label { class: "block",
                        span { class: "mb-1", "fullname" }
                        input {
                            class: "block w-full rounded-md shadow-sm focus:ring focus:ring-opacity-75 focus:dark:ring-violet-600 dark:bg-gray-100",
                            placeholder: "Leroy Jenkins",
                            r#type: "text",
                        }
                    }
                    label { class: "block",
                        span { class: "mb-1", "email" }
                        input {
                            class: "block w-full rounded-md shadow-sm focus:ring focus:ring-opacity-75 focus:dark:ring-violet-600 dark:bg-gray-100",
                            placeholder: "leroy@jenkins.com",
                            r#type: "email",
                        }
                    }
                    label { class: "block",
                        span { class: "mb-1", "message" }
                        textarea {
                            class: "block w-full rounded-md focus:ring focus:ring-opacity-75 focus:dark:ring-violet-600 dark:bg-gray-100",
                            rows: "3",
                        }
                    }
                    button {
                        class: "self-center px-8 py-3 text-lg rounded focus:ring hover:ring focus:ring-opacity-75 dark:bg-violet-600 dark:text-gray-50 focus:dark:ring-violet-600 hover:dark:ring-violet-600",
                        r#type: "button",
                        "Submit"
                    }
                }
            }
        }
    }