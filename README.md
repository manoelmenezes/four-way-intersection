# Four Way Intersection

* There are 4 traffic lights, each with green, yellow and red lights.
   * traffic light for left to right direction.
   * traffic light for right to light direction.
   * traffic light for up to down direction.
   * traffic light for down to up direction.

* Initially, each traffic light will have default values for the time it needs to stay in each state (green, yellow, red).

 * There can be a sensor to detect traffic intensity and change the default values accordingly. For instance, if during the morning rush hours, due to many people going to work in Downtown, the traffic intensity from right to left is high while the intensity from left to right is low, the traffic light that is controlling the transit from right to left would change the defaults to increase the time it stays in green state and decrease the time it stays in red state. The opposite would happen for the left to right direction. This also needs to take into consideration the traffic intensity from up to down and down to up, so if some of those directions are also with high intensity traffic, it does not make sense to change the defaults in the right to left direction.

* There will also be 8 pedestrian lights to signal the pedestrians when they can cross the streets.
 * Up-left-to-right.
 * Left-up-to-down.
 * Down-left-to-right.
 * Left-down-to-up.
 * Up-right-to-left.
 * Right-up-to-down.
 * Down-right-to-left.
 * Right-down-to-up.

* The transition time from Red to Green in traffic lights will be used to allow the pedestrian traffic light to be green when all the traffic lights will be red. After this transition time, the pedestrian light will stay in green for some time, while the traffic light has already transitioned to green. This will only happen for the cases where the traffic lights in the way the pedestrian is crossing are in Red state. For instance, if the pedestrians are crossing from up to down or down to up, this means that the traffic lights in the horizontal directions (left-to-right and right-to-left) are in Red state; the traffic lights in the vertical directions (up-to-down and down-to-up) can already be transitioned to Green state but if some driver is turning to left or right, they need to wait for the pedestrian to cross the street as usual.

* The Four Way Intersection needs to have a controller component that is responsible for:
 * Initialize the state for all lights (traffic and pedestrian). Initially, all pedestrian lights are red, the traffic lights in the horizontal directions are in Green state while the traffic lights in the vertical directions are in Red state.
 * Change the state of each light.
   * After the Green time is over, the controller will set the state to Yellow.
   * After the Yellow time is over, the controller will set the state to Red.
   * As said before, there is a transition time for the traffic lights in the other direction to transition to Green state. On the other hand, as soon as the traffic light in one direction (vertical or horizontal) transitions to Red state, the 4 pedestrian lights that allow crossing this direction will transition to Green state.

