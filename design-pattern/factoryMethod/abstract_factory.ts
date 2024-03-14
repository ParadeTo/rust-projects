interface PointGuard {
    assist(): void
}

class Paul implements PointGuard {
    assist(): void {
        console.log('Paul assist')
    }
}

class Nash implements PointGuard {
    assist(): void {
        console.log('Nash assist')
    }
}

interface CentreForward {
    slamDunk(): void
}

class ONeal implements CentreForward {
    slamDunk(): void {
        console.log('ONeal slam dunk')
    }
}

class YaoMing implements CentreForward {
    slamDunk(): void {
        console.log('YaoMing slam dunk')
    }
}

interface TeamFactory {
    createPointGuard(): PointGuard

    createCentreForward(): CentreForward
}

class RocketTeam implements TeamFactory {
    createCentreForward(): CentreForward {
        return new YaoMing();
    }

    createPointGuard(): PointGuard {
        return new Paul();
    }
}

class LakersTeam implements TeamFactory {
    createCentreForward(): CentreForward {
        return new ONeal();
    }

    createPointGuard(): PointGuard {
        return new Nash();
    }
}

function play(teamFactory: TeamFactory) {
    const centreForward = teamFactory.createCentreForward()
    const pointGuard = teamFactory.createPointGuard()
    pointGuard.assist()
    centreForward.slamDunk()
}

play(new RocketTeam())
play(new LakersTeam())
