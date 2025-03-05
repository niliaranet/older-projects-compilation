
let alice_x = 1
let alice_flipping = false


alice_rabbit_listener.addEventListener(
    "mouseenter",
    _ => { flipCard() },
)

/* this could be done in css */
async function flipCard() {
    if (alice_flipping) {
        console.log("non")
        return
    }

    alice_x *= -1;
    alice_flipping = true;
    await animateAlice("hide");
    console.log("pt1 done");
    await animateAlice("show");
    console.log("pt2 done");
    alice_x *= -1;
    await animateAlice("hide");
    console.log("pt1 done");
    await animateAlice("show");
    console.log("pt2 done");
    alice_rabbit.style.transform = "scaleX(1)";
    alice_flipping = false;
}

async function animateAlice(action) {
    return new Promise((resolve) => {
        let start = Date.now();
        let drawFunction;

        let duration = 100;

        if (action == "hide") {
            drawFunction = function(timePassed, duration) {
                draw(Math.abs(timePassed/duration-1) * -alice_x)
            }

        } else {
            drawFunction = function(timePassed, duration) {
                draw(timePassed/duration * alice_x)
            }
        }
        
        let timer = setInterval(function() {
            let timePassed = Date.now() - start;

            if (timePassed >= duration) {
                clearInterval(timer);
                if (action != "hide") {
                    draw(1);
                }
                resolve();
            }

        drawFunction(timePassed, duration);
        
        }, 20);

        function draw(time) {
            alice_rabbit.style.transform = "scaleX(" + time + ")";
        }

        if (action == "show") {
            draw(alice_flipping)
        }
    })
}
