
function clocktime(hours, minutes) {
    const anglehour = 360 / 12;
    const angleminute = 360 / 60;
    const hourhand = hours * anglehour + minutes / 60 * anglehour;
    const minutehand = minutes * angleminute;
    const angle = Math.abs(hourhand - minutehand);


    //return 360 - angle
    return Math.min(360 - angle, angle)

}

console.log("use the function clocktime() it accepts parameters: hours and minutes")
console.log("for example the function clocktime(7,30):")
console.log(clocktime(7, 30))