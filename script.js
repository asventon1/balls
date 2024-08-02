const canvas = document.getElementById("thing")
const ctx = canvas.getContext("2d");

function dist(x1, y1, x2, y2){
    return Math.sqrt((x2-x1)*(x2-x1)+(y2-y1)*(y2-y1));
}

let SPEED = 5;
let SIZE = 20;
let DAMP = 0.999;
let NUM = 200;
let GRAVITY = 0.2;
let FRAMES = 2;

class Dude { 
    constructor() {
        this.x = Math.random() * 1000
        this.y = Math.random() * 1000
        this.velAngle = Math.random() * 2 * Math.PI;
        this.speed = SPEED;
        this.setVelocityFromAngle();
        this.radius = SIZE;
        this.special = false;
    }
    setVelocityFromAngle() {
        this.xVel = Math.cos(this.velAngle) * this.speed;
        this.yVel = Math.sin(this.velAngle) * this.speed;
    }
    distFrom(dude) {
        return dist(this.x, this.y, dude.x, dude.y);
    }
    updatePosition(dudes) {
        this.x += this.xVel;
        this.y += this.yVel;
        this.yVel += GRAVITY;
    }
    update(dudes) {
        dudes.forEach((v) => {
            let dist = this.distFrom(v);
            if(dist < this.radius * 2 && dist != 0){
                let tanAngle = Math.atan2((v.y-this.y), (v.x-this.x));
                let distanceToMove = this.radius * 2 - dist;
                this.x -= (Math.cos(tanAngle) * distanceToMove)/2;
                this.y -= (Math.sin(tanAngle) * distanceToMove)/2;
                v.x += (Math.cos(tanAngle) * distanceToMove)/2;
                v.y += (Math.sin(tanAngle) * distanceToMove)/2;
                let tanVecY = -(v.x - this.x);
                let tanVecX = v.y - this.y;
                let tanVecLen = Math.sqrt(tanVecY * tanVecY + tanVecX * tanVecX);
                tanVecY /= tanVecLen;
                tanVecX /= tanVecLen;
                let relVelX = v.xVel - this.xVel;
                let relVelY = v.yVel - this.yVel;
                let length = relVelX * tanVecX + relVelY * tanVecY;
                tanVecY *= length;
                tanVecX *= length;
                relVelX -= tanVecX;
                relVelY -= tanVecY;
                this.xVel += relVelX * DAMP;
                this.yVel += relVelY * DAMP;
                v.xVel -= relVelX * DAMP;
                v.yVel -= relVelY * DAMP;
            }
            /*
            if(dist != 0 && v.special) {
                console.log("hello")
                this.xVel += (v.xVel - this.xVel) / dist * 1;
                this.yVel += (v.yVel - this.yVel) / dist * 1;
            }
            */
        });
        
        if(this.x <= this.radius) {
            this.xVel = Math.abs(this.xVel)*DAMP;
            this.x = this.radius;
        } else if (this.x > canvas.width-this.radius){
            this.xVel = -Math.abs(this.xVel)*DAMP;
            this.x = canvas.width-this.radius;
        }
        if(this.y <= this.radius) {
            this.yVel = Math.abs(this.yVel)*DAMP;
            this.y = this.radius;
        } else if (this.y > canvas.width-this.radius){
            this.yVel = -Math.abs(this.yVel)*DAMP;
            this.y = canvas.height-this.radius;
        }
    }
    render() {
        ctx.fillStyle = "red";
        ctx.beginPath();
        ctx.arc(this.x, this.y, this.radius, 0, 2 * Math.PI);
        ctx.fill();
    }
}

let dudes = [];
for(let i = 0; i < NUM; i++){
    dudes.push(new Dude());
}

function updateVars(){
    SPEED = parseFloat(document.getElementById("speedInput").value);
    SIZE = parseFloat(document.getElementById("sizeInput").value);
    DAMP = parseFloat(document.getElementById("dampInput").value);
    NUM = parseInt(document.getElementById("numInput").value);
    GRAVITY = parseFloat(document.getElementById("gravityInput").value);
    FRAMES = parseInt(document.getElementById("framesInput").value);
    dudes = [];
    for(let i = 0; i < NUM; i++){
        dudes.push(new Dude());
    }
}

updateVars();



function update() { 
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    
    for(let i = 0; i < FRAMES; i++) {
        dudes.forEach((v, i) => {
                v.update(dudes);
        });
    }
    dudes.forEach((v, i) => {
        v.updatePosition(dudes);
        v.render();
    });
    window.requestAnimationFrame(update);
}

update();
